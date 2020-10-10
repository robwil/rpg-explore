use macroquad::*;
use macroquad_tiled::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "RPG Explore".to_owned(),
        // create window at scale of 100 pixels to 1 tile.
        window_width: 800,
        window_height: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let map_json_bytes = load_file("assets/maps/larger_map.json").await.expect("failed to load larger_map.json");
    let map_json_str = String::from_utf8(map_json_bytes).expect("failed to convert JSON to utf-8");
    let texture_atlas = load_texture("assets/texture/cave16x16.png").await;
    let map = load_map(
        &map_json_str,
        &[
            ("cave16x16.png", texture_atlas),
        ],
    ).expect("failed to load map");
    let map_height = map.raw_tiled_map.height as f32;
    let map_width = map.raw_tiled_map.width as f32;

    let screen_rect = Rect::new(0., 0., screen_width(), screen_height());
    let mut map_x = 0.;
    let mut map_y = 0.;
    loop {
        clear_background(BLACK);
        
        // note: this renders the top-left 16x16 portion of our map
        // and allows the camera to move with arrow keys
        let camera_rect = Rect::new(map_x, map_y, 15., 15.);
        map.draw_tiles("Tile Layer 1", screen_rect, camera_rect);

        if is_key_down(KeyCode::Left) && map_x > 0. {
            map_x -= 1.;
        }

        if is_key_down(KeyCode::Right) && map_x < map_width-16. {
            map_x += 1.;
        }

        if is_key_down(KeyCode::Up) && map_y > 0. {
            map_y -= 1.;
        }

        if is_key_down(KeyCode::Down) && map_y < map_height-16. {
            map_y += 1.;
        }

        next_frame().await;
    }

}
