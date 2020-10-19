pub const GLOBAL_MULTIPLIER: f32 = 2.; // Global multiplier is the 2x scale we use for all the tiles, both maps and sprites.
pub const GLOBAL_OFFSET_X: f32 = 50.;
pub const GLOBAL_OFFSET_Y: f32 = 50.;

pub const LEVEL_WIDTH: f32 = 14.;
pub const LEVEL_HEIGHT: f32 = 14.;

pub const PLAYER_MOVEMENT_DURATION: f32 = 0.4; // this should be evenly divisible by 4 because we have 4 frames in the movement animation

// these are the sprite frames (aka columns in the sprite sheet) that show player facing different directions
pub const PLAYER_DOWN_FACING_FRAME: f32 = 8.;
pub const PLAYER_UP_FACING_FRAME: f32 = 0.;
pub const PLAYER_RIGHT_FACING_FRAME: f32 = 4.;
pub const PLAYER_LEFT_FACING_FRAME: f32 = 12.;
