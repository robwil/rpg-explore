use crate::components::GridPosition;
use crate::components::Player;
use crate::map::GameMap;
use macroquad::is_key_pressed;
use miniquad::KeyCode;
use specs::Join;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadExpect<'a, GameMap>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, GridPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let try_move_player = |data: Self::SystemData, delta_x: f32, delta_y: f32| {
            let (map, players, mut positions) = data;
            for (_player, position) in (&players, &mut positions).join() {
                let new_x = position.x + delta_x;
                if new_x >= 0. && new_x < map.width {
                    // -1 ?
                    position.x = new_x;
                }
                let new_y = position.y + delta_y;
                if new_y >= 0. && new_y < map.height {
                    // -1?
                    position.y = new_y;
                }
            }
        };

        let mut delta_x = 0.;
        let mut delta_y = 0.;
        if is_key_pressed(KeyCode::Left) {
            delta_x -= 1.;
        }

        if is_key_pressed(KeyCode::Right) {
            delta_x += 1.;
        }

        if is_key_pressed(KeyCode::Up) {
            delta_y -= 1.;
        }

        if is_key_pressed(KeyCode::Down) {
            delta_y += 1.;
        }
        try_move_player(data, delta_x, delta_y);
    }
}
