use crate::text::chunk_text;
use crate::FontAtlas;
use macroquad::prelude::vec2;
use macroquad::prelude::Texture2D;
use macroquad::prelude::Vec2;

pub struct UiState {
    pub font_atlas: FontAtlas,
    pub dialog_box: Option<DialogBox>,
}

#[derive(Default)]
pub struct DialogBox {
    pub position: Vec2,
    pub text_pages: Vec<String>,
    pub current_page: usize,
    pub portrait: Option<Texture2D>,
    pub choices: Vec<String>,
}

// create dialog box entity
// - optional portrait. Make this with ui.canvas().image(...) DrawCanvas
// - optional prompt/selection. Make the selection w/ scrollable list of buttons?
// - based on the above, create the appropriate window layout.
// for now I'm going to use megaUI for this, but in the future might need manual draw_text / draw_rect, in order to create transitions I want
impl UiState {
    pub fn create_dialog_box(
        &mut self,
        text: &str,
        portrait: Option<Texture2D>,
        choices: &[String],
    ) {
        let mut dialog_box = DialogBox {
            position: vec2(10., 500.),
            text_pages: chunk_text(text, 760., 3, &self.font_atlas),
            portrait,
            ..Default::default()
        };

        if !choices.is_empty() {
            dialog_box.choices = choices.to_vec();
        }

        self.dialog_box = Some(dialog_box);
    }
}
