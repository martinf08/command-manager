use crate::app::event_state::EventState;
use crate::app::input::CursorPosition;
use crate::app::state::{StatefulList, TabsState};
use crate::core::config::Config;
use crate::db::db::Db;

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

pub struct App {
    pub config: Config,
    pub tabs: Rc<RefCell<TabsState>>,
    pub db: Db,
    pub event_state: EventState,
    pub inputs: HashMap<String, Vec<char>>,
    pub namespaces: Rc<RefCell<StatefulList<String>>>,
    pub commands: Rc<RefCell<StatefulList<String>>>,
    pub tags: Rc<RefCell<StatefulList<String>>>,
    pub cursor_position: Option<CursorPosition>,
    pub error: Option<String>,
    pub quit: bool,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::new();

        let db = Db::new()?;
        db.init_db()?;

        let namespaces = db.get_namespaces()?;
        let (commands, tags) = db.get_commands_and_tags(None)?;

        Ok(App {
            tabs: Rc::new(RefCell::new(TabsState::new(&config))),
            config,
            db,
            event_state: EventState::default(),
            inputs: HashMap::new(),
            namespaces: Rc::new(RefCell::new(StatefulList::with_items(namespaces))),
            commands: Rc::new(RefCell::new(StatefulList::with_items(commands))),
            tags: Rc::new(RefCell::new(StatefulList::with_items(tags))),
            cursor_position: None,
            error: None,
            quit: false,
        })
    }
}
