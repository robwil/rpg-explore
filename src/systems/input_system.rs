use crate::events::Event;
use crate::game_states::GameState;
use crate::Direction;
use crate::EventQueue;
use macroquad::is_key_down;
use macroquad::is_key_pressed;
use miniquad::KeyCode;
use specs::ReadExpect;
use specs::System;
use specs::WriteExpect;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteExpect<'a, EventQueue>, ReadExpect<'a, GameState>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, game_state) = data;

        if let GameState::AwaitingInput { player_facing } = *game_state {
            let mut direction: Option<Direction> = None;
            if is_key_down(KeyCode::Left) {
                direction = Some(Direction::Left);
            }

            if is_key_down(KeyCode::Right) {
                direction = Some(Direction::Right);
            }

            if is_key_down(KeyCode::Up) {
                direction = Some(Direction::Up);
            }

            if is_key_down(KeyCode::Down) {
                direction = Some(Direction::Down);
            }

            if let Some(direction) = direction {
                event_queue.events.push(Event::PlayerTriesMove(direction));
            }

            if is_key_pressed(KeyCode::Space) {
                event_queue
                    .events
                    .push(Event::PlayerTriesUse(player_facing))
            }
        }
    }
}
