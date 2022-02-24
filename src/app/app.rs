use crate::app::event_state::EventState;
use crate::app::input::CursorPosition;
use crate::app::state::{StatefulList, TabsState};
use crate::core::config::Config;
use crate::db::db::Db;

use std::error::Error;

pub struct App {
    pub commands: StatefulList<String>,
    pub cursor_position: Option<CursorPosition>,
    pub db: Db,
    pub event_state: EventState,
    pub namespaces: StatefulList<String>,
    pub quit: bool,
    pub tabs: TabsState,
    pub tags: StatefulList<String>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::new();

        let db = Db::new()?;
        db.init_db()?;

        let namespaces = db.get_namespaces()?;
        let (commands, tags) = db.get_commands_and_tags(None)?;

        Ok(App {
            commands: StatefulList::with_items(commands),
            cursor_position: None,
            db,
            event_state: EventState::default(),
            namespaces: StatefulList::with_items(namespaces),
            quit: false,
            tabs: TabsState::new(&config),
            tags: StatefulList::with_items(tags),
        })
    }

    pub fn set_commands_tags_from_position(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        let namespace = self.namespaces.items[index].clone();

        let (commands, tags) = self.db.get_commands_and_tags(Some(namespace))?;

        self.commands = StatefulList::with_items(commands);
        self.tags = StatefulList::with_items(tags);

        Ok(())
    }
}
