use macroquad::load_file;
use macroquad::load_texture;
use macroquad_tiled::load_map;
use macroquad_tiled::Map;

pub struct GameMap {
    pub map: Map,
    pub width: f32,
    pub height: f32,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl GameMap {
    pub async fn new() -> GameMap {
        // Load Tiled map definition and necessary textures
        let map_json_bytes = load_file("assets/maps/small_room.json")
            .await
            .expect("failed to load small_room.json");
        let map_json_str =
            String::from_utf8(map_json_bytes).expect("failed to convert JSON to utf-8");
        let texture_atlas = load_texture("assets/texture/rpg_indoor.png").await;
        let map = load_map(&map_json_str, &[("rpg_indoor.png", texture_atlas)])
            .expect("failed to load map");
        let map_height = map.raw_tiled_map.height as f32;
        let map_width = map.raw_tiled_map.width as f32;
        GameMap {
            map,
            width: map_width,
            height: map_height,
            tile_width: 16.,
            tile_height: 16.,
        }
    }
}