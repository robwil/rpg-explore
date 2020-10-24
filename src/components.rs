use crate::constants::*;
use specs::Entity;
use crate::actions::Action;
use macroquad::Texture2D;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Player;

// "marker" struct to wrap Player entity
// RW: I just use this so that we don't have a global Entity on our ECS world.
//     Wrapping it gives the type some identification that makes it harder to mistake what it is.
pub struct PlayerEntity {
    pub entity: Entity
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct GridPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct SpriteDrawable {
    pub texture: Texture2D,
    pub tile_width: f32,    // width for each tile in the texture atlas
    pub tile_height: f32,   // height for each tile in the texture atlas
    pub row: f32,           // which row in the texture atlas to render
    pub current_frame: f32, // which frame (column) in the texture atlas to render
}

#[derive(Component)]
pub struct TriggerActionOnEnter {
    pub action: Action,
}

#[derive(Component)]
pub struct TriggerActionOnExit {
    pub action: Action,
}

#[derive(Component)]
pub struct TriggerActionOnUse {
    pub action: Action,
}

// TODO: eventually deprecate this global game state in favor of entity-level components for each state
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

