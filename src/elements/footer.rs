use crate::App;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

impl App {
    pub fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Quit:q | Select: <space> | Confirm: <enter>")
            .centered()
            .render(area, buf);
    }
}
