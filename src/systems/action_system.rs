use crate::actions::Action;
use crate::components::GridPosition;
use crate::components::Player;
use crate::components::TriggerActionOnEnter;
use crate::components::TriggerActionOnExit;
use crate::components::TriggerActionOnUse;
use crate::events::Event;
use crate::events::EventQueue;
use specs::Join;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, EventQueue>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, TriggerActionOnEnter>,
        ReadStorage<'a, TriggerActionOnExit>,
        ReadStorage<'a, TriggerActionOnUse>,
        WriteStorage<'a, GridPosition>,
    );

    // RW: For now, putting all action handling in one system. This will probably change in the future.
    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, players, enter_triggers, exit_triggers, use_triggers, mut positions) =
            data;

        // Process all events, to determine which actions were triggered
        let mut actions: Vec<Action> = vec![];
        for event in event_queue.events.iter() {
            match event {
                Event::PlayerEntered(pos) => {
                    // look for any triggers that happen on player enter
                    for (enter_action, trigger_pos) in (&enter_triggers, &positions).join() {
                        if trigger_pos == pos {
                            actions.push(enter_action.action.clone());
                        }
                    }
                }
                Event::PlayerExit(pos) => {
                    // look for any triggers that happen on player exit
                    for (exit_action, trigger_pos) in (&exit_triggers, &positions).join() {
                        if trigger_pos == pos {
                            actions.push(exit_action.action.clone());
                        }
                    }
                }
                Event::PlayerTriesUse(direction) => {
                    for (_player, player_position) in (&players, &positions).join() {
                        let use_position = GridPosition {
                            x: player_position.x + direction.get_delta_x(),
                            y: player_position.y + direction.get_delta_y(),
                        };
                        for (use_action, trigger_pos) in (&use_triggers, &positions).join() {
                            if *trigger_pos == use_position {
                                actions.push(use_action.action.clone());
                            }
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
                Action::PrintMessage(message) => {
                    println!("PRINT MESSAGE action: {}", message);
                }
            }
        }
    }
}
