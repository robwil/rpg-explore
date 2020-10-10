use macroquad::Texture2D;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
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