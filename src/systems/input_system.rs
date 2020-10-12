use crate::events::Event;
use crate::EventQueue;
use crate::game_states::GameState;
use macroquad::is_key_down;
use miniquad::KeyCode;
use specs::ReadExpect;
use specs::System;
use specs::WriteExpect;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteExpect<'a, EventQueue>,
        ReadExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, game_state) = data;

        if *game_state != GameState::AwaitingInput {
            return;
        }

        let mut delta_x = 0.;
        let mut delta_y = 0.;
        if is_key_down(KeyCode::Left) {
            delta_x -= 1.;
        }

        if is_key_down(KeyCode::Right) {
            delta_x += 1.;
        }

        if is_key_down(KeyCode::Up) {
            delta_y -= 1.;
        }

        if is_key_down(KeyCode::Down) {
            delta_y += 1.;
        }

        if delta_x != 0. || delta_y != 0. {
            event_queue.events.push(Event::PlayerTriesMove{delta_x, delta_y});
        }
    }
}
