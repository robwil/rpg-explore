use crate::components::GridPosition;
use crate::components::SpriteDrawable;
use crate::constants::GLOBAL_MULTIPLIER;
use crate::constants::GLOBAL_OFFSET_X;
use crate::constants::GLOBAL_OFFSET_Y;
use crate::constants::LEVEL_HEIGHT;
use crate::constants::LEVEL_WIDTH;
use crate::map::GameMap;
use macroquad::draw_texture_ex;
use macroquad::DrawTextureParams;
use macroquad::Rect;
use macroquad::Vec2;
use macroquad::WHITE;
use specs::Join;
use specs::ReadExpect;
use specs::{ReadStorage, System};

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadExpect<'a, GameMap>,
        ReadStorage<'a, GridPosition>,
        ReadStorage<'a, SpriteDrawable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, positions, drawables) = data;

        // draw map
        let level_rect = Rect::new(0., 0., LEVEL_WIDTH - 1., LEVEL_HEIGHT - 1.);
        let draw_dest_rect = Rect::new(
            GLOBAL_OFFSET_X,
            GLOBAL_OFFSET_Y,
            map.tile_width * map.width * GLOBAL_MULTIPLIER,
            map.tile_height * map.height * GLOBAL_MULTIPLIER,
        );
        map.map
            .draw_tiles("Tile Layer 1", draw_dest_rect, level_rect);

        // draw any SpriteDrawables with GridPosition
        for (drawable, position) in (&drawables, &positions).join() {
            draw_texture_ex(
                drawable.texture,
                // x position is simply the current grid position * map tile width (plus the global modifiers)
                GLOBAL_OFFSET_X + position.x * map.tile_width * GLOBAL_MULTIPLIER,
                // y position is the current grid position * map tile height, but then we subtract half of
                // map tile height and sprite height to make it look like the sprite is at bottom of the tile instead of its center
                GLOBAL_OFFSET_Y + position.y * map.tile_height * GLOBAL_MULTIPLIER
                    - map.tile_height / 2.
                    - drawable.tile_height / 2.,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        drawable.tile_width * drawable.current_frame, // select current animation frame
                        drawable.tile_height * drawable.row,          // for current texture row
                        drawable.tile_width,
                        drawable.tile_height,
                    )),
                    dest_size: Some(Vec2::new(
                        drawable.tile_width * GLOBAL_MULTIPLIER,
                        drawable.tile_height * GLOBAL_MULTIPLIER,
                    )),
                    ..Default::default()
                },
            );
        }
    }
}
