use crate::ui::UiState;
use macroquad::prelude::glam;
use megaui_macroquad::draw_window;
use megaui_macroquad::megaui::hash;
use megaui_macroquad::megaui::widgets::Label;
use megaui_macroquad::WindowParams;
use specs::ReadExpect;
use specs::System;

pub struct UiSystem;

impl<'a> System<'a> for UiSystem {
    type SystemData = (ReadExpect<'a, UiState>,);

    fn run(&mut self, data: Self::SystemData) {
        let (ui_state,) = data;

        if let Some(dialog_box) = &ui_state.dialog_box {
            draw_window(
                hash!(),
                dialog_box.position,
                // TODO: extract constants for this static size
                glam::vec2(780., 120.),
                WindowParams {
                    // TODO: add optional title / character name
                    movable: false,
                    titlebar: false,
                    ..Default::default()
                },
                |ui| {
                    Label::new(&dialog_box.text_pages[dialog_box.current_page])
                        // TODO: extract constant
                        .multiline(24.)
                        .position(None)
                        .ui(ui);
                },
            );
        }
    }
}
