use std::error::Error;
use crate::app::add::Add;
use crate::app::input::CursorPosition;
use crate::app::state::{StatefulList, TabsState};
use crate::Db;
use crossterm::event::Event;
use tui::widgets::ListState;
use crate::app::event_state::EventState;
use crate::core::config::Config;


pub struct App<'a> {
    pub commands: StatefulList<String>,
    pub cursor_position: Option<CursorPosition>,
    pub db: Db,
    pub event_state: EventState,
    pub namespaces: StatefulList<String>,
    pub tabs: TabsState,
    pub tags: StatefulList<String>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::new();

        let db = Db::new()?;

        let namespaces = db.get_namespaces()?;
        let (commands, tags) = db.get_commands_and_tags(None)?;

        Ok(App {
            commands: StatefulList::with_items(commands),
            cursor_position: None,
            db,
            event_state: EventState::new(),
            namespaces: StatefulList::with_items(namespaces),
            tabs: TabsState::new(&config),
            tags: StatefulList::with_items(tags),
        })
    }
}
