use crate::app::input::CursorPosition;
use crate::App;
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

pub fn set_cursor_position(app: &mut App, f: &mut Frame<impl Backend>, rect: Rect, input: String) {
    if app.cursor_position.is_none() {
        app.cursor_position = Some(CursorPosition::new(
            rect.x as usize,
            rect.y as usize,
            rect.width as usize,
            input.clone(),
        ));
    }

    f.set_cursor(
        app.cursor_position.as_ref().unwrap().x as u16,
        app.cursor_position.as_ref().unwrap().y as u16,
    )
}
