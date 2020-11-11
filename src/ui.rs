use crate::FONT_SIZE;
use crate::constants::DIALOG_BOX_HEIGHT;
use crate::constants::DIALOG_BOX_WIDTH;
use crate::constants::UI_TEXTURE_CARET;
use crate::megaui::widgets::Texture;
use crate::text::chunk_text;
use crate::FontAtlas;
use macroquad::prelude::glam;
use macroquad::prelude::vec2;
use macroquad::prelude::Texture2D;
use macroquad::prelude::Vec2;
use megaui_macroquad::draw_window;
use megaui_macroquad::megaui::hash;
use megaui_macroquad::megaui::widgets::Label;
use megaui_macroquad::megaui::Vector2;
use megaui_macroquad::WindowParams;
use std::hash::Hash;
use std::hash::Hasher;

pub struct UiState {
    pub font_atlas: FontAtlas,
    pub dialog_box: Option<DialogBox>,
}

#[derive(Debug, Clone, Default)]
pub struct DialogBoxConf {
    pub message: String,
    pub title: Option<String>,
    pub portrait: Option<Texture2D>,
    pub choices: Vec<String>,
}

#[derive(Default, Debug)]
pub struct DialogBox {
    position: Vec2,
    text_pages: Vec<String>,
    current_page: usize,
    title: Option<String>,
    portrait: Option<Texture2D>,
    choices: Vec<String>,
}

impl Hash for DialogBox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hasher must include any elements of self that will change the WindowParams passed to draw_window
        self.title.is_some().hash(state);
    }
}

impl DialogBox {
    pub fn render(&self) {
        let mut window_params = WindowParams {
            movable: false,
            titlebar: false,
            ..Default::default()
        };
        if let Some(title) = &self.title {
            window_params.label = title.to_owned();
            window_params.titlebar = true;
        }
        draw_window(
            hash!(self),
            self.position,
            glam::vec2(DIALOG_BOX_WIDTH, DIALOG_BOX_HEIGHT),
            window_params,
            |ui| {
                Label::new(&self.text_pages[self.current_page])
                    .multiline(FONT_SIZE as f32)
                    .ui(ui);

                // TODO: Portrait. Make this with Texture like below caret

                

                // if showing any page but last page, show continue caret
                if self.current_page < self.text_pages.len() - 1 {
                    Texture::new(UI_TEXTURE_CARET)
                        .size(22., 22.)
                        // bottom-right corner
                        .position(Some(Vector2::new(DIALOG_BOX_WIDTH - 32., DIALOG_BOX_HEIGHT - 32.)))
                        .ui(ui);
                } else {
                    // TODO: choices, show on last page
                }
                
            },
        );
    }
    // advances to next page of text and returns true.
    // returns false if there is no next page
    pub fn advance_to_next_page(&mut self) -> bool {
        if self.current_page + 1 < self.text_pages.len() {
            self.current_page += 1;
            return true;
        }
        false
    }
}

impl UiState {
    // Returns true if the UI is currently engaged, meaning that the typical player inputs should be directed toward the UI as opposed to the game world.
    pub fn is_engaged(&self) -> bool {
        self.dialog_box.is_some()
    }

    // create dialog box entity
    // - optional portrait.
    // - optional prompt/selection. Make the selection w/ scrollable list of buttons?
    // - based on the above, create the appropriate window layout.
    // for now I'm going to use megaUI for this, but in the future might need manual draw_text / draw_rect, in order to create transitions I want
    pub fn create_dialog_box(&mut self, conf: DialogBoxConf) {
        let mut lines_per_page = 3;
        if conf.title.is_some() {
            lines_per_page = 2;
        }
        let mut dialog_box = DialogBox {
            position: vec2(10., 500.),
            text_pages: chunk_text(&conf.message, 760., lines_per_page, &self.font_atlas),
            title: conf.title,
            portrait: conf.portrait,
            ..Default::default()
        };

        if !conf.choices.is_empty() {
            dialog_box.choices = conf.choices.to_vec();
        }

        self.dialog_box = Some(dialog_box);
    }

    pub fn dialog_next_page(&mut self) {
        if let Some(dialog_box) = &mut self.dialog_box {
            // advancing beyond the last page maens we should close the dialog box, by removing it from self
            if !dialog_box.advance_to_next_page() {
                self.dialog_box = None;
            }
        }
    }
}
