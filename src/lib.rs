pub mod app;
mod fixtures;
mod ui;

use crossterm::event;
use crossterm::event::{Event, KeyCode};

use std::io;
use std::time::Duration;
use tui::backend::Backend;
use tui::Terminal;
use ui::ui;

use crate::app::App;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<Option<String>> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(None),
                    KeyCode::Left => {
                        app.folders.unselect();
                        app.tabs.previous();
                    }
                    KeyCode::Right => {
                        app.folders.unselect();
                        app.tabs.next();
                    }
                    KeyCode::Down => app.folders.next(),
                    KeyCode::Up => app.folders.previous(),
                    KeyCode::Enter => {}
                    _ => {}
                }
            }
        }
    }
}
