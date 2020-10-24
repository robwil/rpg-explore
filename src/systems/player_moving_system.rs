use crate::components::GridPosition;
use crate::components::Player;
use crate::components::SpriteDrawable;
use crate::constants::*;
use crate::events::Event;
use crate::events::EventQueue;
use crate::game_states::GameState;
use crate::map::GameMap;
use macroquad::get_frame_time;
use specs::Join;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;
use specs::WriteStorage;

// This system is responsible for player movement.
// Currently, this includes:
// 1) listening for PlayerTriesMove event and initiate GameState::PlayerMoving if moving to a valid location
// 2) handling the animation that occurs when the game enters GameState::PlayerMoving
// 3) firing events for PlayerExit and PlayerEnter for the old and new positions
pub struct PlayerMovingSystem;

impl<'a> System<'a> for PlayerMovingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, GameState>,
        WriteExpect<'a, EventQueue>,
        ReadExpect<'a, GameMap>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, SpriteDrawable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let delta_time = get_frame_time();

        let (mut game_state, mut event_queue, map, players, mut positions, mut drawables) = data;

        // Handle events: PlayerTriesMove
        let mut new_events: Vec<Event> = vec![];
        for event in event_queue.events.iter() {
            if let Event::EntityTriesMove(direction) = event {
                for (_player, drawable, position) in (&players, &mut drawables, &positions).join() {
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
                    // perform actual move (will be handled below)
                    if moving {
                        *game_state = GameState::PlayerMoving {
                            delta_x,
                            delta_y,
                            direction: *direction,
                        };
                        new_events.push(Event::PlayerExit(*position));
                    } else {
                        // even if player didn't move, we at least need to change their current frame to match their possible change in direction
                        drawable.current_frame = direction.get_player_facing_frame();
                        *game_state = GameState::AwaitingInput {
                            player_facing: *direction,
                        };
                    }
                }
            }
        }

        // Handle player that is already moving.
        if let GameState::PlayerMoving {
            delta_x,
            delta_y,
            direction,
        } = *game_state
        {
            // TODO: The below code is pretty complicated. It'd be a lot clearer what was going on if we had a Tween class.
            // That might also allow removing the duplication because the distinct part of each of the below is mostly the directionality differences.
            for (_player, position, drawable) in (&players, &mut positions, &mut drawables).join() {
                if delta_y > 0. {
                    // down
                    // the difference between 1 and current delta_y is how much we've moved so far.
                    // we use that to determine the current animation frame and to tween the actual movement of position
                    let elapsed_duration = (1. - delta_y) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame = PLAYER_DOWN_FACING_FRAME
                        + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.y += movement_this_frame;
                    let mut new_delta_y = delta_y - movement_this_frame;
                    if new_delta_y < 0. {
                        // finished moving
                        new_delta_y = 0.;
                        drawable.current_frame = PLAYER_DOWN_FACING_FRAME;
                        position.y = position.y.round();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: 0.,
                        delta_y: new_delta_y,
                        direction,
                    };
                } else if delta_y < 0. {
                    // up
                    let elapsed_duration = (1. - -delta_y) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame = PLAYER_UP_FACING_FRAME
                        + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.y -= movement_this_frame;
                    let mut new_delta_y = delta_y + movement_this_frame;
                    if new_delta_y > 0. {
                        // finished moving
                        new_delta_y = 0.;
                        drawable.current_frame = PLAYER_UP_FACING_FRAME;
                        position.y = position.y.round();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: 0.,
                        delta_y: new_delta_y,
                        direction,
                    };
                } else if delta_x > 0. {
                    // right
                    let elapsed_duration = (1. - delta_x) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame = PLAYER_RIGHT_FACING_FRAME
                        + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.x += movement_this_frame;
                    let mut new_delta_x = delta_x - movement_this_frame;
                    if new_delta_x < 0. {
                        // finished moving
                        new_delta_x = 0.;
                        drawable.current_frame = PLAYER_RIGHT_FACING_FRAME;
                        position.x = position.x.round();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: new_delta_x,
                        delta_y: 0.,
                        direction,
                    };
                } else if delta_x < 0. {
                    // left
                    let elapsed_duration = (1. - -delta_x) * PLAYER_MOVEMENT_DURATION + delta_time;
                    let movement_this_frame = delta_time / PLAYER_MOVEMENT_DURATION;
                    drawable.current_frame = PLAYER_LEFT_FACING_FRAME
                        + (elapsed_duration / PLAYER_MOVEMENT_DURATION * 4.).floor();
                    position.x -= movement_this_frame;
                    let mut new_delta_x = delta_x + movement_this_frame;
                    if new_delta_x > 0. {
                        // finished moving
                        new_delta_x = 0.;
                        drawable.current_frame = PLAYER_LEFT_FACING_FRAME;
                        position.x = position.x.round();
                    }
                    *game_state = GameState::PlayerMoving {
                        delta_x: new_delta_x,
                        delta_y: 0.,
                        direction,
                    };
                } else {
                    // once delta_x and delta_y are 0, the movement is over
                    *game_state = GameState::AwaitingInput {
                        player_facing: direction,
                    };
                    new_events.push(Event::PlayerEntered(*position));
                }
            }
        }

        // Add any events that occurred from TryMove or actual movement
        event_queue.new_events.append(&mut new_events);
    }
}
