use crate::ui::UiState;

use macroquad::prelude::is_key_pressed;
use macroquad::prelude::KeyCode;

use specs::System;
use specs::WriteExpect;

pub struct UiSystem;

impl<'a> System<'a> for UiSystem {
    type SystemData = (WriteExpect<'a, UiState>,);

    fn run(&mut self, data: Self::SystemData) {
        let (mut ui_state,) = data;

        if let Some(dialog_box) = &ui_state.dialog_box {
            // Render the existing dialog
            dialog_box.render();

            // Handle input to advance pages
            if is_key_pressed(KeyCode::Space) {
                ui_state.dialog_next_page();
            }
        }
    }
}
