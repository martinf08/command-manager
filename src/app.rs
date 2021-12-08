use crate::fixtures::generate_data;
use tui::widgets::ListState;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        TabsState { titles, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        self.index = (self.index + self.titles.len() - 1) % self.titles.len();
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> Self {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn current(&self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => 0,
        }
    }
}

pub struct CommandStates {
    pub items: Vec<Vec<(String, String)>>,
    pub index: usize,
    pub state: ListState,
}

impl CommandStates {
    pub fn new(items: Vec<Vec<(String, String)>>) -> Self {
        CommandStates {
            items: items.clone(),
            index: 0,
            state: ListState::default(),
        }
    }

    pub fn set_position(&mut self, index: usize) {
        self.index = index;
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub folders: StatefulList<String>,
    pub commands: CommandStates,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        let (folders, commands) = generate_data();
        App {
            title,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            folders: StatefulList::with_items(folders),
            commands: CommandStates::new(commands),
        }
    }
}
