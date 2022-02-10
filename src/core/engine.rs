use crate::core::parser::{KeyParser, ParserResult};
use crate::ui::ui;
use crate::App;

use crossterm::event;
use crossterm::event::Event;
use std::time::Duration;
use tui::backend::Backend;
use tui::Terminal;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> ParserResult {
    app.set_current_selected_tab(true);

    loop {
        if app.quit {
            return Ok(None);
        }

        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let result = KeyParser::parse_event(key, &mut app)?;

                if let Some(key_parser_result) = result {
                    return Ok(Some(key_parser_result));
                }
            }
        }
    }
}
