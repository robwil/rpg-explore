use crate::components::AwaitingInputState;
use crate::components::EntityMovingState;
use crate::components::FacingDirection;
use crate::components::GridPosition;
use crate::components::SpriteDrawable;
use crate::constants::*;
use crate::events::Event;
use crate::events::EventQueue;
use crate::map::GameMap;
use crate::PlayerEntity;
use macroquad::get_frame_time;
use specs::Entities;
use specs::Entity;
use specs::Join;
use specs::ReadExpect;
use specs::System;
use specs::WriteExpect;
use specs::WriteStorage;

// This system is responsible for all character movement.
// Currently, this includes:
// 1) listening for EntityTriesMove event and puts that entity in EntityMovingState if moving to a valid location
// 2) handling the animation that occurs when an entity is in EntityMovingState
// 3) fires events for PlayerExit and PlayerEnter for the old and new positions, if the entity that moved was the PlayerEntity

pub struct CharacterMovingSystem;

impl<'a> System<'a> for CharacterMovingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, EventQueue>,
        ReadExpect<'a, GameMap>,
        ReadExpect<'a, PlayerEntity>,
        Entities<'a>,
        WriteStorage<'a, AwaitingInputState>,
        WriteStorage<'a, EntityMovingState>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, SpriteDrawable>,
        WriteStorage<'a, FacingDirection>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let delta_time = get_frame_time();

        let (
            mut event_queue,
            map,
            player_entity,
            entities,
            mut awaiting_input_states,
            mut entity_moving_states,
            mut positions,
            mut drawables,
            mut facing_directions,
        ) = data;

        // Handle events: EntityTriesMove
        let mut new_events: Vec<Event> = vec![];
        for event in event_queue.events.iter() {
            if let Event::EntityTriesMove(entity, direction) = event {
                if let (Some(drawable), Some(position)) =
                    (drawables.get_mut(*entity), positions.get(*entity))
                {
                    let delta_x: f32 = direction.get_delta_x();
                    let delta_y: f32 = direction.get_delta_y();
                    let mut moving = false;
                    let new_x = position.x + delta_x;
                    let new_y = position.y + delta_y;
                    println!(
                        "current position = {:?}, trying new position = {},{}, facing: {:?}",
                        *position, new_x, new_y, direction
                    );
                    // ensure they don't leave map
                    if new_x >= 0. && new_x < map.width && new_y >= 0. && new_y < map.height {
                        moving = true;
                    }
                    // check if the new location is actually somewhere we can move
                    if map.is_blocked(new_x, new_y) {
                        moving = false;
                    }

                    // Regardless of actually moving, their attempt to move has changed their facing direction
                    if let Some(facing_direction) = facing_directions.get_mut(*entity) {
                        *facing_direction = FacingDirection {
                            direction: *direction,
                        };
                    }
                    drawable.current_frame = direction.get_player_facing_frame();

                    // if the move was successful, perform actual move (will be handled below) by adding EntityMovingState to the entity
                    if moving {
                        if entity.id() == player_entity.entity.id() {
                            awaiting_input_states.remove(*entity);
                        }
                        entity_moving_states
                            .insert(
                                *entity,
                                EntityMovingState {
                                    delta_x,
                                    delta_y,
                                    direction: *direction,
                                },
                            )
                            .expect("failed to insert entity moving state");
                        new_events.push(Event::PlayerExit(*position));
                    }
                }
            }
        }

        // Handle player that is already moving.
        // TODO: The below code is pretty complicated. It'd be a lot clearer what was going on if we had a Tween class.
        // That might also allow removing the duplication because the distinct part of each of the below is mostly the directionality differences.
        let mut entities_done_moving: Vec<Entity> = vec![];
        for (entity, moving_state, position, drawable) in (
            &entities,
            &mut entity_moving_states,
            &mut positions,
            &mut drawables,
        )
            .join()
        {
            if moving_state.delta_y > 0. {
                // down
                // the difference between 1 and current delta_y is how much we've moved so far.
                // we use that to determine the current animation frame and to tween the actual movement of position
                let elapsed_duration =
                    (1. - moving_state.delta_y) * PLAYER_MOVEMENT_DURATION + delta_time;
                let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                drawable.current_frame = PLAYER_DOWN_FACING_FRAME
                    + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                position.y += movement_this_frame;
                let mut new_delta_y = moving_state.delta_y - movement_this_frame;
                if new_delta_y < 0. {
                    // finished moving
                    new_delta_y = 0.;
                    drawable.current_frame = PLAYER_DOWN_FACING_FRAME;
                    position.y = position.y.round();
                }
                moving_state.delta_y = new_delta_y;
            } else if moving_state.delta_y < 0. {
                // up
                let elapsed_duration =
                    (1. - -moving_state.delta_y) * PLAYER_MOVEMENT_DURATION + delta_time;
                let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                drawable.current_frame = PLAYER_UP_FACING_FRAME
                    + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                position.y -= movement_this_frame;
                let mut new_delta_y = moving_state.delta_y + movement_this_frame;
                if new_delta_y > 0. {
                    // finished moving
                    new_delta_y = 0.;
                    drawable.current_frame = PLAYER_UP_FACING_FRAME;
                    position.y = position.y.round();
                }
                moving_state.delta_y = new_delta_y;
            } else if moving_state.delta_x > 0. {
                // right
                let elapsed_duration =
                    (1. - moving_state.delta_x) * PLAYER_MOVEMENT_DURATION + delta_time;
                let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                drawable.current_frame = PLAYER_RIGHT_FACING_FRAME
                    + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                position.x += movement_this_frame;
                let mut new_delta_x = moving_state.delta_x - movement_this_frame;
                if new_delta_x < 0. {
                    // finished moving
                    new_delta_x = 0.;
                    drawable.current_frame = PLAYER_RIGHT_FACING_FRAME;
                    position.x = position.x.round();
                }
                moving_state.delta_x = new_delta_x;
            } else if moving_state.delta_x < 0. {
                // left
                let elapsed_duration =
                    (1. - -moving_state.delta_x) * PLAYER_MOVEMENT_DURATION + delta_time;
                let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                drawable.current_frame = PLAYER_LEFT_FACING_FRAME
                    + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                position.x -= movement_this_frame;
                let mut new_delta_x = moving_state.delta_x + movement_this_frame;
                if new_delta_x > 0. {
                    // finished moving
                    new_delta_x = 0.;
                    drawable.current_frame = PLAYER_LEFT_FACING_FRAME;
                    position.x = position.x.round();
                }
                moving_state.delta_x = new_delta_x;
            } else {
                // once delta_x and delta_y are 0, the movement is over
                entities_done_moving.push(entity);

                // TODO: make this more generic to non-player characters
                if entity.id() == player_entity.entity.id() {
                    // only actual player has Awaiting Input state.
                    // NPCs probably have a different default state, which we'll need to handle later
                    awaiting_input_states
                        .insert(player_entity.entity, AwaitingInputState {})
                        .expect("failed to insert player AwaitingInputState");
                    // and only player currently tracks enter events
                    new_events.push(Event::PlayerEntered(*position));
                }
            }
        }

        // remove EntityMovingState from any entities that are done their moving animation
        for entity in entities_done_moving {
            entity_moving_states.remove(entity);
        }

        // Add any events that occurred from TryMove or actual movement
        event_queue.new_events.append(&mut new_events);
    }
}
