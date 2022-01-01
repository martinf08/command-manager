use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Clear};
use tui::Frame;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let split_y = (100 - percent_y) / 2;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(split_y),
                Constraint::Percentage(percent_y),
                Constraint::Percentage(split_y),
            ]
            .as_ref(),
        )
        .split(r);

    let split_x = (100 - percent_x) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(split_x),
                Constraint::Percentage(percent_x),
                Constraint::Percentage(split_x),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn get_border_style_from_selected_status(selected: bool) -> Style {
    if selected {
        return Style::default().fg(Color::White);
    }

    Style::default().fg(Color::DarkGray)
}

pub fn get_highlight_style() -> Style {
    Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Red)
        .bg(Color::Gray)
}

pub fn get_popup_layout<B>(f: &mut Frame<B>, rect: Rect, margin_ratio: Option<u8>) -> Vec<Rect>
where
    B: Backend,
{
    let block = Block::default()
        .title("Add entry")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let area = centered_rect(70, 20, rect);

    f.render_widget(Clear, area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref());

    if margin_ratio.is_some() {
        let layout = layout.margin(area.height / margin_ratio.unwrap() as u16);

        return layout.split(area);
    }

    layout.split(area)
}