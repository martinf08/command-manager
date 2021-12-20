use crate::input::key_parser::{KeyParser, KeyParserResult};
use crate::ui::ui;
use crate::App;

use crossterm::event;
use crossterm::event::Event;
use std::time::Duration;
use tui::backend::Backend;
use tui::Terminal;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> KeyParserResult {
    app.tabs.current_selected = true;

    loop {
        if app.quit {
            return Ok(None);
        }

        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let result = KeyParser::parse_key(key, &mut app);

                if result.is_ok() {
                    if let Some(key_parser_result) = result.unwrap() {
                        return Ok(Some(key_parser_result));
                    }
                }
            }
        }
    }
}
