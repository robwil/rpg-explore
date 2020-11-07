use crate::events::Event;
use crate::AwaitingInputState;
use crate::Direction;
use crate::EventQueue;
use crate::PlayerEntity;
use macroquad::input::is_key_down;
use macroquad::input::is_key_pressed;
use macroquad::prelude::KeyCode;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteExpect<'a, EventQueue>,
        ReadStorage<'a, AwaitingInputState>,
        ReadExpect<'a, PlayerEntity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, awaiting_input_states, player_entity) = data;

        if let Some(_player_awaiting_input) = awaiting_input_states.get(player_entity.entity) {
            let mut direction: Option<Direction> = None;
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                direction = Some(Direction::Left);
            }

            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                direction = Some(Direction::Right);
            }

            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                direction = Some(Direction::Up);
            }

            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                direction = Some(Direction::Down);
            }

            if let Some(direction) = direction {
                event_queue
                    .events
                    .push(Event::EntityTriesMove(player_entity.entity, direction));
            }

            if is_key_pressed(KeyCode::Space) {
                event_queue.events.push(Event::PlayerTriesUse())
            }
        }
    }
}
