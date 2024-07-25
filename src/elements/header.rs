use crate::App;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};

impl App {
    pub fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Select files to convert")
            .bold()
            .centered()
            .render(area, buf);
    }
}
