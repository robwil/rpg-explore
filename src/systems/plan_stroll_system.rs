use crate::components::Direction;
use crate::components::Strolling;
use crate::components::WaitingState;
use crate::events::Event;
use crate::events::EventQueue;
use crate::EntityMovingState;
use macroquad::get_frame_time;
use quad_rand as qrand;
use specs::Entities;
use specs::Join;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;
use specs::WriteStorage;

// This system is responsible for planning (basic "AI") of all Strolling entities.
// Currently, this includes:
// 1) managing the entity WaitingState for some amount of time
// 2) choosing a random direction and trying to move there using an EntityTriesMove event and letting movement system handle that

pub struct PlanStrollSystem;

impl<'a> System<'a> for PlanStrollSystem {
    type SystemData = (
        WriteExpect<'a, EventQueue>,
        Entities<'a>,
        ReadStorage<'a, Strolling>,
        ReadStorage<'a, EntityMovingState>,
        WriteStorage<'a, WaitingState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, entities, strollings, entity_moving_states, mut waiting_states) =
            data;
        let delta_time = get_frame_time();

        for (entity, strolling) in (&entities, &strollings).join() {
            match (
                waiting_states.get_mut(entity),
                entity_moving_states.get(entity),
            ) {
                // waiting and not moving
                (Some(waiting_state), None) => {
                    if waiting_state.remaining_wait_seconds - delta_time <= 0. {
                        // finished waiting, so pick a random direction to move
                        let direction = match qrand::gen_range(0, 4) {
                            0 => Direction::Up,
                            1 => Direction::Down,
                            2 => Direction::Left,
                            _ => Direction::Right,
                        };
                        // start movement and stop waiting state
                        event_queue
                            .events
                            .push(Event::EntityTriesMove(entity, direction));
                        waiting_states.remove(entity);
                    } else {
                        waiting_state.remaining_wait_seconds -= delta_time;
                    }
                }
                // no waiting OR moving state, means it's this system's job to set up WaitingState
                // this happens at beginning of game or when previous movement has finished
                (None, None) => {
                    waiting_states
                        .insert(
                            entity,
                            WaitingState {
                                remaining_wait_seconds: qrand::gen_range(
                                    0.,
                                    strolling.max_pause_seconds,
                                ),
                            },
                        )
                        .expect("failed to insert waiting state for strolling entity");
                }
                // otherwise, the entity is already moving so it's being handled by movement system
                _ => (),
            };
        }
    }
}
