use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use cm::{App, run_app, write_command_in_terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let history_file = std::env::var("HISTORY_FILE").expect("HISTORY_FILE env not set, provide a path to a history file. ex : HISTORY_FILE=/home/user/.history");
    let app = App::init_from_history_file(history_file);
    let result = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match &result {
        Ok(Some(_)) => (),
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
        _ => return Ok(()),
    }

    // write selected input in terminal
    write_command_in_terminal(result.unwrap().unwrap())?;

    Ok(())
}
