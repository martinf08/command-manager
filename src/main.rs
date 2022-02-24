mod app;
mod core;
mod db;
mod ui;
mod widget;

use crate::app::app::App;
use crate::core::Engine;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;
    Engine::run(app)?;

    Ok(())
}
