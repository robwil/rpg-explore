use crate::actions::Action;
use crate::constants::*;
use macroquad::Texture2D;
use specs::Entity;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Player;

// "marker" struct to wrap Player entity
// RW: I just use this so that we don't have a global Entity on our ECS world.
//     Wrapping it gives the type some identification that makes it harder to mistake what it is.
pub struct PlayerEntity {
    pub entity: Entity,
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
pub struct FacingDirection {
    pub direction: Direction,
}

// a strolling entity will pause for some amount of time, then move in a random direction, and repeat
#[derive(Component)]
pub struct Strolling {
    pub max_pause_seconds: f32,
}

//////////////////////////////////////////
//
// Components used with Actions system
//
//////////////////////////////////////////

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

//////////////////////////////////////////
//
// Components used as States (i.e. state machine)
//
//////////////////////////////////////////

#[derive(Component)]
pub struct AwaitingInputState {} // should only be used by Player entity

#[derive(Component, Debug)]
pub struct WaitingState{
    pub remaining_wait_seconds: f32,
}

#[derive(Component)]
pub struct EntityMovingState {
    pub delta_x: f32,
    pub delta_y: f32,
    pub direction: Direction,
}

//////////////////////////////////////////
//
// Helper data structs / logic used by multiple components
//
//////////////////////////////////////////

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
    pub fn get_character_facing_frame(&self) -> f32 {
        match self {
            Direction::Left => CHARACTER_LEFT_FACING_FRAME,
            Direction::Right => CHARACTER_RIGHT_FACING_FRAME,
            Direction::Up => CHARACTER_UP_FACING_FRAME,
            Direction::Down => CHARACTER_DOWN_FACING_FRAME,
        }
    }
}
