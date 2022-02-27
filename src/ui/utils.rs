use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use crate::App;
use crate::app::input::CursorPosition;

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
