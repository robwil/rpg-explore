use crate::actions::Action;
use crate::components::GridPosition;
use crate::components::Player;
use crate::components::SpriteDrawable;
use crate::components::TriggerActionOnEnter;
use crate::components::TriggerActionOnExit;
use crate::components::TriggerActionOnUse;
use crate::events::EventQueue;
use crate::game_states::Direction;
use crate::game_states::GameState;
use crate::map::GameMap;
use crate::systems::ActionSystem;
use crate::systems::InputSystem;
use crate::systems::PlayerMovingSystem;
use crate::systems::RenderingSystem;
use macroquad::*;
use specs::RunNow;
use specs::{Builder, World, WorldExt};

mod actions;
mod components;
mod constants;
mod events;
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
    world.register::<TriggerActionOnEnter>();
    world.register::<TriggerActionOnExit>();
    world.register::<TriggerActionOnUse>();

    // Insert global resources
    let map = GameMap::new().await;
    world.insert(map);
    world.insert(GameState::AwaitingInput {
        player_facing: Direction::Down,
    });
    world.insert(EventQueue {
        ..Default::default()
    });

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
    // Top door
    world
        .create_entity()
        .with(GridPosition { x: 11., y: 2. })
        .with(TriggerActionOnEnter {
            action: Action::Teleport(GridPosition { x: 10., y: 11. }),
        })
        .build();
    // Bottom door
    world
        .create_entity()
        .with(GridPosition { x: 10., y: 12. })
        .with(TriggerActionOnEnter {
            action: Action::Teleport(GridPosition { x: 11., y: 3. }),
        })
        .build();
    // Starting position
    world
        .create_entity()
        .with(GridPosition { x: 9., y: 3. })
        .with(TriggerActionOnExit {
            action: Action::PrintMessage("left start position".to_owned()),
        })
        .build();
    // Useable urn
    world
        .create_entity()
        .with(GridPosition { x: 10., y: 3. })
        .with(TriggerActionOnUse {
            action: Action::PrintMessage("the urn is full of snakes!".to_owned()),
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

        let mut action_system = ActionSystem {};
        action_system.run_now(&world);

        rendering_system.run_now(&world);

        world.maintain();

        let mut event_queue = world.write_resource::<EventQueue>();
        if !event_queue.events.is_empty() {
            println!("current events: {:?}", event_queue.events);
        }
        if !event_queue.new_events.is_empty() {
            println!("new events: {:?}", event_queue.new_events);
        }
        event_queue.events = (*event_queue.new_events).to_vec();
        event_queue.new_events.clear();

        next_frame().await;
    }
}
