use crate::core::parser::ParserResult;
use crate::App;
use tui::backend::Backend;
use tui::Terminal;

pub mod engine;
mod parser;

pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: App) -> ParserResult {
    engine::run_app(terminal, app)
}
