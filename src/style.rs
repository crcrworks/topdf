use ratatui::style::{Color, Modifier, Style};

pub const HEADER_STYLE: Style = Style::new()
    .fg(Color::from_u32(0x00232A2E))
    .bg(Color::from_u32(0x0083C092));

pub const BORDER_STYLE: Style = Style::new().fg(Color::from_u32(0x00D3C6AA));
pub const NORMAL_ROW_BG: Color = Color::from_u32(0x00343F44);
pub const ALT_ROW_BG_COLOR: Color = Color::from_u32(0x00343F44);
pub const SELECTED_STYLE: Style = Style::new()
    .bg(Color::from_u32(0x00475258))
    .add_modifier(Modifier::BOLD);
pub const TEXT_FG_COLOR: Color = Color::from_u32(0x00D3C6AA);
pub const COMPLETED_TEXT_FG_COLOR: Color = Color::from_u32(0x0083C092);

pub const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
