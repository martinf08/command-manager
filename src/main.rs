mod app;
mod cmd;
mod core;
mod db;
mod fixtures;
mod ui;
mod widget;

use crate::app::app::App;
use crate::cmd::Cmd;
use crate::core::run;

use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    db::init_db(true)?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen /*, EnableMouseCapture*/)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Command Manager");
    let result = run(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
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
