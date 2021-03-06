use crate::core::cmd::Cmd;
use crate::App;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod cmd;
pub mod config;
mod engine;
mod parser;

pub struct Engine;

impl Engine {
    pub fn run(app: App) -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = engine::run_app(&mut terminal, app);

        // restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        match &result {
            Ok(Some((cmd_line, _tag))) => {
                Cmd::create_and_run(cmd_line)?;
            }
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
            _ => return Ok(()),
        }

        Ok(())
    }
}
