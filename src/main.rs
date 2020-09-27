use macroquad::*;

#[macroquad::main("RPG Explore")]
async fn main() {
    let texture: Texture2D = load_texture("assets/texture/grass_tile.png").await;
    let tile_size = texture.width(); // being lazy here since it's a square
    let tiles_per_row = (screen_width() / texture.width()).ceil() as usize;
    let tiles_per_column = (screen_height() / texture.height()).ceil() as usize;
    
    loop {
        clear_background(BLACK);
        for i in 0..tiles_per_row {
            for j in 0..tiles_per_column {
                draw_texture(
                    texture,
                    0. + i as f32 * tile_size,
                    0. + j as f32 * tile_size,
                    WHITE,
                );
            }
        }
        next_frame().await
    }
}
