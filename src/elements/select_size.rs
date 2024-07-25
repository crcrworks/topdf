use ratatui::{
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
    Frame,
};

use crate::{style::*, unit::MM, App};

pub struct SizeOptionList {
    pub options: Vec<SizeOption>,
    pub state: ListState,
}

pub struct SizeOption {
    pub name: String,
    pub width: MM,
    pub height: MM,
}

impl SizeOption {
    pub fn new(option: SizeOption) -> Self {
        Self {
            name: option.name,
            width: option.width,
            height: option.height,
        }
    }
}

impl App {
    pub fn render_popup(&mut self, f: &mut Frame, area: Rect) {
        let floating_area = App::centered_rect(30, 40, area);

        let block = Block::bordered()
            .border_style(BORDER_STYLE)
            .bg(NORMAL_ROW_BG)
            .title(Line::raw("Select Size").centered());

        let items: Vec<ListItem> = self
            .size_option_list
            .options
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

        f.render_widget(Clear, floating_area);
        StatefulWidget::render(
            list,
            floating_area,
            f.buffer_mut(),
            &mut self.size_option_list.state,
        )
    }
}

impl From<&SizeOption> for ListItem<'_> {
    fn from(size_option: &SizeOption) -> Self {
        let line = Line::styled(size_option.name.to_string(), TEXT_FG_COLOR);
        ListItem::new(line)
    }
}
