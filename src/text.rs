use crate::FontAtlas;

pub fn chunk_text(
    text: &str,
    wrap_width: f32,
    lines_per_chunk: usize,
    font_atlas: &FontAtlas,
) -> Vec<String> {
    let wrapped_text = wrap_text(text, wrap_width, font_atlas);
    let lines: Vec<&str> = wrapped_text.split('\n').collect();
    lines
        .chunks(lines_per_chunk)
        .map(|f| f.join("\n"))
        .collect()
}

pub fn wrap_text(text: &str, wrap_width: f32, font_atlas: &FontAtlas) -> String {
    let words = text.split(' ');
    let space_width = character_width(font_atlas, ' ');
    let mut lines = vec![];
    let mut current_line = vec![];
    let mut current_pixels = 0.;
    for word in words {
        let word_width = word_width(font_atlas, word);
        if word_width + current_pixels + (current_line.len() as f32 * space_width) > wrap_width {
            lines.push(current_line.join(" "));
            current_line = vec![];
            current_pixels = word_width;
            current_line.push(word);
        } else {
            current_pixels += word_width;
            current_line.push(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line.join(" "))
    }
    lines.join("\n")
}

fn character_width(font_atlas: &FontAtlas, character: char) -> f32 {
    if let Some(font_data) = font_atlas.character_infos.get(&character) {
        let font_data = font_data.scale(font_atlas.font_size as f32);
        let width = font_data.left_padding + font_data.size.0 + font_data.right_padding;
        return width;
    }
    0.
}

fn word_width(font_atlas: &FontAtlas, word: &str) -> f32 {
    word.chars()
        .map(|c| character_width(font_atlas, c))
        .sum::<f32>()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn font_atlas() -> FontAtlas {
        FontAtlas::new(
            &include_bytes!("../../megaui/assets/ProggyClean.ttf")[..],
            20,
            FontAtlas::ascii_character_list(),
        )
        .unwrap()
    }

    #[test]
    fn test_character_widths() {
        // ProggyClean is a fixed width font, so all characters should be same width.
        let expected_width = 10.769231;
        assert_eq!(character_width(&font_atlas(), 'a'), expected_width);
        assert_eq!(character_width(&font_atlas(), 'b'), expected_width);
        assert_eq!(character_width(&font_atlas(), 'H'), expected_width);
        assert_eq!(character_width(&font_atlas(), '!'), expected_width);
        // space is slightly different width
        let expected_width = 10.769232;
        assert_eq!(character_width(&font_atlas(), ' '), expected_width);
    }

    #[test]
    fn test_word_widths() {
        // ProggyClean is a fixed width font, so all characters should be same width.
        let expected_char_width = 10.769231;
        assert_eq!(word_width(&font_atlas(), "abc"), 3. * expected_char_width);
        assert_eq!(
            word_width(&font_atlas(), "Hello!"),
            6. * expected_char_width
        );
    }

    #[test]
    fn test_wrap_texts() {
        assert_eq!(wrap_text("abc", 600., &font_atlas()), "abc");
        assert_eq!(wrap_text("Here is some long text that should go on to the next line. You see, this game is starting to get some story.", 600., &font_atlas()), 
            "Here is some long text that should go on to the next\nline. You see, this game is starting to get some story.");
        assert_eq!(wrap_text("Here is some long text that should go on to the next line. You see, this game is starting to get some story.", 100., &font_atlas()), 
            "Here is\nsome long\ntext that\nshould go\non to the\nnext\nline. You\nsee, this\ngame is\nstarting\nto get\nsome\nstory.");
    }

    #[test]
    fn test_chunk_text() {
        assert_eq!(chunk_text("Here is some long text that should go on to the next line. You see, this game is starting to get some story.", 100., 3, &font_atlas()), 
            vec!["Here is\nsome long\ntext that", "should go\non to the\nnext", "line. You\nsee, this\ngame is", "starting\nto get\nsome", "story."]);
    }
}
