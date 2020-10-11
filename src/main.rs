use crate::components::GridPosition;
use crate::components::Player;
use crate::components::SpriteDrawable;
use crate::game_states::GameState;
use crate::map::GameMap;
use crate::systems::InputSystem;
use crate::systems::PlayerMovingSystem;
use crate::systems::RenderingSystem;
use macroquad::*;
use specs::RunNow;
use specs::{Builder, World, WorldExt};

mod components;
mod constants;
mod game_states;
mod map;
mod systems;

fn window_conf() -> Conf {
    Conf {
        window_title: "RPG Explore".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Setup specs world
    let mut world = World::new();
    world.register::<GridPosition>();
    world.register::<SpriteDrawable>();
    world.register::<Player>();

    let map = GameMap::new().await;
    world.insert(map);
    world.insert(GameState::AwaitingInput);

    // Create entities
    let character_texture = load_texture("assets/texture/walk_cycle.png").await;
    world
        .create_entity()
        .with(Player {})
        .with(GridPosition { x: 9., y: 3. })
        .with(SpriteDrawable {
            texture: character_texture,
            tile_width: 16.,
            tile_height: 24.,
            row: 0.,
            current_frame: 8.,
        })
        .build();

    let mut rendering_system = RenderingSystem {
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        let mut input_system = InputSystem {};
        input_system.run_now(&world);

        let mut player_moving_system = PlayerMovingSystem {};
        player_moving_system.run_now(&world);

        rendering_system.run_now(&world);

        world.maintain();

        next_frame().await;
    }
}
