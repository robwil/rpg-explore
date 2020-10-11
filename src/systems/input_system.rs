use crate::components::GridPosition;
use crate::components::Player;
use crate::game_states::GameState;
use crate::map::GameMap;
use macroquad::is_key_down;
use miniquad::KeyCode;
use specs::Join;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadExpect<'a, GameMap>,
        WriteExpect<'a, GameState>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, GridPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut game_state, players, positions) = data;

        if *game_state != GameState::AwaitingInput {
            return;
        }

        let mut try_move_player = |delta_x: f32, delta_y: f32| {
            for (_player, position) in (&players, &positions).join() {
                let mut moving = false;
                let new_x = position.x + delta_x;
                let new_y = position.y + delta_y;
                println!("current position = {:?}", *position);
                if new_x >= 0. && new_x < map.width {
                    moving = true;
                }
                if new_y >= 0. && new_y < map.height {
                    moving = true;
                }
                if moving {
                    *game_state = GameState::PlayerMoving { delta_x, delta_y };
                }
            }
        };

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
            try_move_player(delta_x, delta_y);
        }
    }
}
