use crate::constants::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameState {
    AwaitingInput {
        player_facing: Direction,
    },
    PlayerMoving {
        delta_x: f32,
        delta_y: f32,
        direction: Direction,
    },
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn get_delta_x(&self) -> f32 {
        match self {
            Direction::Left => -1.,
            Direction::Right => 1.,
            Direction::Up => 0.,
            Direction::Down => 0.,
        }
    }
    pub fn get_delta_y(&self) -> f32 {
        match self {
            Direction::Left => 0.,
            Direction::Right => 0.,
            Direction::Up => -1.,
            Direction::Down => 1.,
        }
    }
    pub fn get_player_facing_frame(&self) -> f32 {
        match self {
            Direction::Left => PLAYER_LEFT_FACING_FRAME,
            Direction::Right => PLAYER_RIGHT_FACING_FRAME,
            Direction::Up => PLAYER_UP_FACING_FRAME,
            Direction::Down => PLAYER_DOWN_FACING_FRAME,
        }
    }
}
