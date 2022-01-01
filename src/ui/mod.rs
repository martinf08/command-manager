pub mod ui;
mod utils;

use crate::App;

use tui::backend::Backend;
use tui::Frame;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    ui::ui(f, app);
}
