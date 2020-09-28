use macroquad::*;

const MAP_WIDTH: f32 = 8.;
const MAP_HEIGHT: f32 = 7.;

fn window_conf() -> Conf {
    Conf {
        window_title: "RPG Explore".to_owned(),
        // create window at scale of 100 pixels to 1 tile.
        window_width: 800,
        window_height: 700,
        ..Default::default()
    }
}

fn get_map_tile(map: &Vec<usize>, x: usize, y: usize) -> usize {
    map[x + MAP_WIDTH as usize * y]
}

#[macroquad::main(window_conf)]
async fn main() {
    let textures = vec![
        load_texture("assets/tile/tiles_00.png").await,
        load_texture("assets/tile/tiles_01.png").await,
        load_texture("assets/tile/tiles_02.png").await,
        load_texture("assets/tile/tiles_03.png").await,
        load_texture("assets/tile/tiles_04.png").await,
        load_texture("assets/tile/tiles_05.png").await,
        load_texture("assets/tile/tiles_06.png").await,
        load_texture("assets/tile/tiles_07.png").await,
        load_texture("assets/tile/tiles_08.png").await,
        load_texture("assets/tile/tiles_09.png").await,
        load_texture("assets/tile/tiles_10.png").await
    ];
    let map: Vec<usize> = vec![
        1,1,1,1,5,6,7,1,
        1,1,1,1,5,6,7,1,
        1,1,1,1,5,6,7,1,
        3,3,3,3,11,6,7,1,
        9,9,9,9,9,9,10,1,
        1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,2,3,
    ];
    

    let tile_size = textures[0].width(); // being lazy here since it's a square, both width and height are same

    // TODO: figure this out -- right now this works but leads to an inverted view somehow.
    // build camera such that we are zoomed in on the 8x7 grid of tiles
    // set_camera(Camera2D::from_display_rect(Rect::new(0., 0., MAP_WIDTH * tile_size, MAP_HEIGHT * tile_size)));
    
    loop {
        clear_background(BLACK);
        for x in 0..MAP_WIDTH as usize {
            for y in 0..MAP_HEIGHT as usize {
                draw_texture(
                    textures[get_map_tile(&map, x, y) - 1],
                    0. + x as f32 * tile_size,
                    0. + y as f32 * tile_size,
                    WHITE,
                );
            }
        }
        next_frame().await
    }
}
