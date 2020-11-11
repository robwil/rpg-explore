use crate::actions::Action;
use crate::components::FacingDirection;
use crate::components::GridPosition;
use crate::components::PlayerEntity;
use crate::components::TriggerActionOnEnter;
use crate::components::TriggerActionOnExit;
use crate::components::TriggerActionOnUse;
use crate::events::Event;
use crate::events::EventQueue;
use crate::ui::DialogBox;
use crate::ui::UiState;
use specs::Entities;
use specs::Join;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::Write;
use specs::WriteExpect;
use specs::WriteStorage;

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, EventQueue>,
        ReadExpect<'a, PlayerEntity>,
        ReadStorage<'a, TriggerActionOnEnter>,
        ReadStorage<'a, TriggerActionOnExit>,
        ReadStorage<'a, TriggerActionOnUse>,
        ReadStorage<'a, FacingDirection>,
        WriteStorage<'a, GridPosition>,
        WriteExpect<'a, UiState>,
    );

    // RW: For now, putting all action handling in one system. This will probably change in the future.
    fn run(&mut self, data: Self::SystemData) {
        let (
            event_queue,
            player_entity,
            enter_triggers,
            exit_triggers,
            use_triggers,
            facing_directions,
            mut positions,
            mut ui_state,
        ) = data;

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
                Event::PlayerTriesUse() => {
                    if let (Some(player_position), Some(player_facing_direction)) = (
                        positions.get(player_entity.entity),
                        facing_directions.get(player_entity.entity),
                    ) {
                        let use_position = GridPosition {
                            x: player_position.x + player_facing_direction.direction.get_delta_x(),
                            y: player_position.y + player_facing_direction.direction.get_delta_y(),
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
                    if let Some(player_position) = positions.get_mut(player_entity.entity) {
                        player_position.x = pos.x;
                        player_position.y = pos.y;
                    }
                }
                Action::ShowDialog(message) => {
                    ui_state.create_dialog_box(&message, None, &[]);
                }
                Action::PrintMessage(message) => {
                    println!("PRINT MESSAGE action: {}", message);
                }
            }
        }
    }
}
