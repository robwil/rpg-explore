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

    loop {
        clear_background(BLACK);
        let dest_rect = Rect::new(0., 0., screen_width(), screen_height());
        // note: this renders the top-left 16x16 portion of our map
        let source_rect = Rect::new(0., 0., 15., 15.);
        map.draw_tiles("Tile Layer 1", dest_rect, source_rect);
        next_frame().await;
    }

}
