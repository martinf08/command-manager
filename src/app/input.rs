use crate::App;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

#[derive(Debug)]
pub struct CursorPosition {
    initial_x: usize,
    initial_y: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub input: String,
}

impl CursorPosition {
    pub fn new(x: usize, y: usize, width: usize, input: String) -> Self {
        CursorPosition {
            initial_x: x,
            initial_y: y,
            x,
            y,
            width,
            input,
        }
    }

    fn inc(&mut self) {
        if self.input.len() % self.width.saturating_sub(1) != 0 {
            self.x += 1;
        } else {
            self.x = self.initial_x;
            self.y += 1;
        }
    }

    fn dec(&mut self) {
        if self.input.len() == 0 {
            self.x = self.initial_x;
            return;
        }

        if self.input.len() % self.width.saturating_sub(1) != 0 && self.x > self.initial_x {
            self.x = self.x.saturating_sub(1);
        } else {
            if self.x > self.initial_x {
                self.x = self.x.saturating_sub(1);
            } else {
                if self.y > self.initial_y {
                    self.y = self.y.saturating_sub(1);
                    self.x = self.width.saturating_sub(1);
                }
            }
        }
    }

    pub fn push_inc(&mut self, c: char) {
        self.input.push(c);
        self.inc();
    }

    pub fn pop_dec(&mut self) {
        self.input.pop();
        self.dec();
    }

    pub fn set_cursor_position(
        app: &mut App,
        f: &mut Frame<impl Backend>,
        rect: Rect,
        input: String,
    ) {
        if app.cursor_position.is_none() {
            app.cursor_position = Some(CursorPosition::new(
                (rect.x + 1) as usize,
                (rect.y + 1) as usize,
                rect.width as usize,
                input.clone(),
            ));
        }

        f.set_cursor(
            app.cursor_position.as_ref().unwrap().x as u16,
            app.cursor_position.as_ref().unwrap().y as u16,
        )
    }
}
