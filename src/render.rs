use crate::{App, Scene};

use ratatui::layout::{Constraint, Layout};
use ratatui::Frame;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

impl App {
    pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .margin(10)
        .areas(area);

        let [list_area, _item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
    }
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let area = f.size();

    f.render_widget(&mut *app, area);
    match app.scene {
        Scene::PageFormat => app.render_popup(f, area),
        Scene::Name => app.render_input_name(f, area),
        _ => {}
    }
}
