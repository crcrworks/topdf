use crate::{
    file::{FileItem, Status},
    style::*,
    App,
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols,
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Padding, StatefulWidget},
};

impl App {
    pub fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Files").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::new(2, 2, 1, 1));

        let items: Vec<ListItem> = self
            .file_list
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let color = alternate_colors(i);
                ListItem::from(item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.file_list.state)
    }
}

impl From<&FileItem> for ListItem<'_> {
    fn from(value: &FileItem) -> Self {
        let line = match value.status {
            Status::Unchecked => {
                Line::styled(format!("  {}", value.path.to_string_lossy()), TEXT_FG_COLOR)
            }
            Status::Checked => Line::styled(
                format!(" ✓ {}", value.path.to_string_lossy()),
                COMPLETED_TEXT_FG_COLOR,
            ),
        };
        ListItem::new(line)
    }
}
