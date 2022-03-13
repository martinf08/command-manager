mod app;
mod core;
mod db;
mod ui;

use crate::app::app::App;
use crate::core::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;
    Engine::run(app)?;

    Ok(())
}
