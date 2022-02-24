use crate::core::parser::{KeyParser, ParserResult};
use crate::ui::ui;
use crate::App;
use std::io::Stdout;

use crossterm::event;
use crossterm::event::Event;
use std::time::Duration;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub fn run_app(
    mut terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> ParserResult {
    app.tabs.current_selected = true;

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
