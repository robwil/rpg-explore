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

    let map_json_bytes = load_file("assets/maps/map.json").await.expect("failed to load map.json");
    let map_json_str = String::from_utf8(map_json_bytes).expect("failed to convert JSON to utf-8");
    let texture_atlas = load_texture("assets/texture/atlas.png").await;
    let map = load_map(
        &map_json_str,
        &[
            ("atlas.png", texture_atlas),
        ],
    ).expect("failed to load map");

    // Lets draw the whole map full screen
    // Default macroquad camera is pixel perfect with (0, 0) in top left corner and (screen_width(), screen_height()) on bottom right
    let dest_rect = Rect::new(0., 0., screen_width(), screen_height());

    // We used only part of tiled canvas to create our first level
    // So lets draw only that part of the canvas
    // Area is hardcoded for now, but we will use the technique of drawing parts of tiled canvas
    // to jump through level sections in the future
    let source_rect = Rect::new(0., 0., 7., 6.);

    loop {
        clear_background(BLACK);
        map.draw_tiles("Tile Layer 1", dest_rect, source_rect);
        next_frame().await;
    }

}
