use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
    Frame,
};

use ratatui::style::Stylize;

use crate::{style::*, App};

#[derive(Default)]
pub struct Input {
    pub value: String,
    pub character_index: usize,
    pub messages: Vec<String>,
}

impl Input {
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.value.insert(index, new_char);
        self.move_cursor_right();
    }
    pub fn byte_index(&mut self) -> usize {
        self.value
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.value.len())
    }
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.value.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.value.chars().skip(current_index);

            self.value = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.messages.push(self.value.clone());
        self.value.clear();
        self.reset_cursor();
    }
}

impl App {
    pub fn render_input_name(&mut self, f: &mut Frame, area: Rect) {
        let input_area = App::centered_rect(30, 8, area);

        let input = Paragraph::new(self.input.value.as_str()).block(
            Block::bordered()
                .border_style(BORDER_STYLE)
                .bg(NORMAL_ROW_BG)
                .title("File Name .pdf"),
        );

        f.render_widget(input, input_area);

        f.set_cursor(
            input_area.x + self.input.character_index as u16 + 1,
            input_area.y + 1,
        );
    }
}
