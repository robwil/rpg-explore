use crate::actions::Action;
use crate::components::GridPosition;
use crate::components::Player;
use crate::components::TriggerActionOnEnter;
use crate::events::Event;
use crate::events::EventQueue;
use specs::Join;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    type SystemData = (
        ReadExpect<'a, EventQueue>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, TriggerActionOnEnter>,
        WriteStorage<'a, GridPosition>,
    );

    // RW: For now, putting all action handling in one system. This will probably change in the future.
    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, players, enter_triggers, mut positions) = data;

        // Process all events, to determine which actions were triggered
        let mut actions: Vec<Action> = vec![];
        for event in event_queue.events.iter() {
            match event {
                Event::PlayerEntered(pos) => {
                    // look for any triggers that happen on player enter
                    for (enter_action, trigger_pos) in (&enter_triggers, &positions).join() {
                        if trigger_pos == pos {
                            actions.push(enter_action.action);
                        }
                    }
                }
                _ => (),
            }
        }

        // Process any actions that were just triggered
        for action in actions {
            println!("Processing action: {:?}", action);
            match action {
                Action::Teleport(pos) => {
                    for (_player, position) in (&players, &mut positions).join() {
                        position.x = pos.x;
                        position.y = pos.y;
                    }
                }
            }
        }
    }
}
