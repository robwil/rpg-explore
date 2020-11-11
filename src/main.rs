use crate::constants::FONT_SIZE;
use crate::actions::Action;
use crate::components::AwaitingInputState;
use crate::components::BlocksMovement;
use crate::components::Direction;
use crate::components::EntityMovingState;
use crate::components::FacingDirection;
use crate::components::GridPosition;
use crate::components::Player;
use crate::components::PlayerEntity;
use crate::components::SpriteDrawable;
use crate::components::Strolling;
use crate::components::TriggerActionOnEnter;
use crate::components::TriggerActionOnExit;
use crate::components::TriggerActionOnUse;
use crate::constants::UI_TEXTURE_CARET;
use crate::events::EventQueue;
use crate::map::GameMap;
use crate::megaui::Style;
use crate::systems::ActionSystem;
use crate::systems::CharacterMovingSystem;
use crate::systems::InputSystem;
use crate::systems::PlanStrollSystem;
use crate::systems::RenderingSystem;
use crate::systems::UiSystem;
use crate::ui::DialogBoxConf;
use crate::ui::UiState;
use macroquad::prelude::*;
use megaui::Color;
use megaui::FontAtlas;
use megaui_macroquad::set_megaui_texture;
use megaui_macroquad::set_ui_style;
use megaui_macroquad::{
    draw_megaui,
    megaui::{self},
    set_font_atlas,
};
use specs::DispatcherBuilder;
use specs::{Builder, World, WorldExt};

mod actions;
mod components;
mod constants;
mod events;
mod map;
mod systems;
mod text;
mod ui;
mod util;

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
    // load global textures
    let texture = load_texture("assets/texture/continue_caret.png").await;
    set_megaui_texture(UI_TEXTURE_CARET, texture);

    // setup UI style
    let font_bytes = &include_bytes!("../assets/fonts/Roboto-Bold.ttf")[..];
    let font_atlas =
        FontAtlas::new(font_bytes, FONT_SIZE, FontAtlas::ascii_character_list()).unwrap();
    set_font_atlas(font_atlas);
    set_ui_style(Style {
        title_height: 32.,
        margin: 5.,
        window_background_focused: Color::from_rgb(0, 0, 150),
        focused_title: Color::from_rgb(255, 255, 255),
        focused_text: Color::from_rgb(255, 255, 255),
        ..Default::default()
    });
    // need to recreate font_atlas that got moved above, so we can use it below
    let font_atlas =
        FontAtlas::new(font_bytes, FONT_SIZE, FontAtlas::ascii_character_list()).unwrap();

    // Setup specs world
    let mut world = World::new();
    // We only need to explicitly register the components used by Player entity, the rest get setup by dispatcher.setup(...)
    world.register::<Player>();
    world.register::<BlocksMovement>();
    world.register::<GridPosition>();
    world.register::<SpriteDrawable>();
    world.register::<FacingDirection>();
    world.register::<AwaitingInputState>();

    // Create player entity
    let character_texture = load_texture("assets/texture/walk_cycle.png").await;
    let player_entity = world
        .create_entity()
        .with(Player {})
        .with(BlocksMovement {})
        // .with(GridPosition { x: 9., y: 3. })
        .with(GridPosition { x: 2., y: 4. })
        .with(SpriteDrawable {
            texture: character_texture,
            tile_width: 16.,
            tile_height: 24.,
            row: 0.,
            current_frame: 8.,
        })
        .with(FacingDirection {
            direction: Direction::Down,
        })
        .with(AwaitingInputState {})
        .build();

    // Insert global resources
    let map = GameMap::new().await;
    world.insert(map);
    world.insert(EventQueue {
        ..Default::default()
    });
    world.insert(PlayerEntity {
        entity: player_entity,
    });
    world.insert(UiState {
        font_atlas,
        dialog_box: None,
    });

    // Dispatcher setup will register all systems and do other setup
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(PlanStrollSystem, "plan_stroll", &[])
        .with(
            CharacterMovingSystem,
            "character_moving",
            &["input", "plan_stroll"],
        )
        .with(ActionSystem, "action", &[])
        .with(
            RenderingSystem {
                ..Default::default()
            },
            "rendering",
            &[],
        )
        .with(UiSystem, "ui", &["rendering"])
        .build();
    dispatcher.setup(&mut world);

    // Create non-player entities

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
        .with(BlocksMovement {})
        .with(TriggerActionOnUse {
            action: Action::ShowSimpleDialog("The urn is full of snakes!".to_owned()),
        })
        .build();
    // Banner
    world
        .create_entity()
        .with(GridPosition { x: 8., y: 2. })
        .with(TriggerActionOnUse {
            action: Action::ShowSimpleDialog("When, O Catiline, do you mean to cease abusing our patience? How long is that madness of yours still to mock us? When is there to be an end of that unbridled audacity of yours, swaggering about as it does now? Do not the nightly guards placed on the Palatine Hill -- do not the watches posted throughout the city -- does not the alarm of the people, and the union of all good men -- does not the precaution taken of assembling the senate in this most defensible place -- do not the looks and countenances of this venerable body here present, have any effect upon you? Do you not feel that your plans are detected? Do you not see that your conspiracy is already arrested and rendered powerless by the knowledge which every one here possesses of it? What is there that you did last night, what the night before -- where is it that you were -- who was there that you summoned to meet you -- what design was there which was adopted by you, with which you think that any one of us is unacquainted?".to_owned()),
        })
        .build();
    // Standing NPC
    world
        .create_entity()
        .with(GridPosition { x: 3., y: 4. })
        .with(BlocksMovement {})
        .with(SpriteDrawable {
            texture: character_texture,
            tile_width: 16.,
            tile_height: 24.,
            row: 2.,
            current_frame: 8.,
        })
        .with(FacingDirection {
            direction: Direction::Down,
        })
        .with(TriggerActionOnUse {
            action: Action::ShowDialog(DialogBoxConf {
                message: "Welcome to town. Do you like it so far?".to_owned(),
                title: Some("Innkeeper".to_owned()),
                ..Default::default()
            }),
        })
        .build();
    // Strolling NPC
    world
        .create_entity()
        .with(GridPosition { x: 6., y: 8. })
        .with(BlocksMovement {})
        .with(SpriteDrawable {
            texture: character_texture,
            tile_width: 16.,
            tile_height: 24.,
            row: 4.,
            current_frame: 8.,
        })
        .with(FacingDirection {
            direction: Direction::Down,
        })
        .with(Strolling {
            max_pause_seconds: 3.,
        })
        .build();

    loop {
        clear_background(BLACK);

        // run ECS systems
        dispatcher.dispatch(&world);
        world.maintain();

        // handle events
        let mut event_queue = world.write_resource::<EventQueue>();
        if !event_queue.events.is_empty() {
            println!("current events: {:?}", event_queue.events);
        }
        if !event_queue.new_events.is_empty() {
            println!("new events: {:?}", event_queue.new_events);
        }
        event_queue.events = (*event_queue.new_events).to_vec();
        event_queue.new_events.clear();

        draw_megaui();

        next_frame().await;
    }
}
