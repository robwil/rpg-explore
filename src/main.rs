use macroquad::*;
use macroquad_tiled::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "RPG Explore".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

// Global multiplier is the 2x scale we use for all the tiles, both maps and sprites.
const GLOBAL_MULTIPLIER: f32 = 2.;
const GLOBAL_OFFSET_X: f32 = 50.;
const GLOBAL_OFFSET_Y: f32 = 50.;
// Viewport represents the camera viewport size.
const VIEWPORT_WIDTH: f32 = 12.;
const VIEWPORT_HEIGHT: f32 = 12.;

#[macroquad::main(window_conf)]
async fn main() {
    // Load map
    let map_json_bytes = load_file("assets/maps/small_room.json")
        .await
        .expect("failed to load small_room.json");
    let map_json_str = String::from_utf8(map_json_bytes).expect("failed to convert JSON to utf-8");
    let texture_atlas = load_texture("assets/texture/rpg_indoor.png").await;
    let map =
        load_map(&map_json_str, &[("rpg_indoor.png", texture_atlas)]).expect("failed to load map");
    let map_height = map.raw_tiled_map.height as f32;
    let map_width = map.raw_tiled_map.width as f32;
    let map_tile_width = 16.;
    let map_tile_height = 16.;

    // Load character walk_cycle
    // each frame of the walk animation is 16x24
    let character_texture = load_texture("assets/texture/walk_cycle.png").await;
    let character_tile_width = 16.;
    let character_tile_height = 24.;

    // this keeps a 100px border around the "map". this looks nicer for rooms but for world map would be undesirable.
    let draw_dest_rect = Rect::new(
        GLOBAL_OFFSET_X,
        GLOBAL_OFFSET_Y,
        map_tile_width * map_width * GLOBAL_MULTIPLIER,
        map_tile_height * map_height * GLOBAL_MULTIPLIER,
    );
    let mut camera_x = 0.;
    let mut camera_y = 0.;
    let mut character_x = 10.;
    let mut character_y = 2.;
    loop {
        clear_background(BLACK);

        let camera_rect = Rect::new(
            camera_x,
            camera_y,
            VIEWPORT_WIDTH - 1.,
            VIEWPORT_HEIGHT - 1.,
        );
        map.draw_tiles("Tile Layer 1", draw_dest_rect, camera_rect);

        draw_texture_ex(
            character_texture,
            GLOBAL_OFFSET_X + character_x * map_tile_width * GLOBAL_MULTIPLIER,
            GLOBAL_OFFSET_Y + character_y * map_tile_height * GLOBAL_MULTIPLIER
                - map_tile_height / 2.
                - character_tile_height / 2.,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    character_tile_width * 8.,
                    character_tile_height * 0.,
                    character_tile_width,
                    character_tile_height,
                )),
                dest_size: Some(Vec2::new(
                    character_tile_width * GLOBAL_MULTIPLIER,
                    character_tile_height * GLOBAL_MULTIPLIER,
                )),
                ..Default::default()
            },
        );

        if is_key_pressed(KeyCode::Left) && character_x > 0. {
            character_x -= 1.;
        }

        if is_key_pressed(KeyCode::Right) && character_x < map_width - 1. {
            character_x += 1.;
        }

        if is_key_pressed(KeyCode::Up) && character_y > 0. {
            character_y -= 1.;
        }

        if is_key_pressed(KeyCode::Down) && character_y < map_height - 1. {
            character_y += 1.;
        }

        next_frame().await;
    }
}
