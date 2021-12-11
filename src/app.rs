use crate::fixtures::generate_data;
use tui::widgets::ListState;

pub trait State {
    fn next(&mut self);
    fn previous(&mut self);
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub current_selected: bool,
}

impl<'a> State for TabsState<'a> {
    fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        self.index = (self.index + self.titles.len() - 1) % self.titles.len();
    }
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        TabsState {
            titles,
            index: 0,
            current_selected: false,
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub current_selected: bool,
}

fn get_next_state_to_select(state: &ListState, item_count: usize) -> Option<usize> {
    match state.selected() {
        Some(i) => {
            if i >= item_count - 1 {
                Some(0)
            } else {
                Some(i + 1)
            }
        }
        None => Some(0),
    }
}

fn get_previous_state_to_select(state: &ListState, item_count: usize) -> Option<usize> {
    match state.selected() {
        Some(i) => {
            if i == 0 {
                Some(item_count - 1)
            } else {
                Some(i - 1)
            }
        }
        None => Some(0),
    }
}

impl<T> State for StatefulList<T> {
    fn next(&mut self) {
        self.state
            .select(get_next_state_to_select(&self.state, self.items.len()));
    }

    fn previous(&mut self) {
        self.state
            .select(get_previous_state_to_select(&self.state, self.items.len()));
    }
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> Self {
        StatefulList {
            state: ListState::default(),
            items,
            current_selected: false,
        }
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
    pub current_selected: bool,
}

impl State for CommandStates {
    fn next(&mut self) {
        self.state.select(get_next_state_to_select(
            &self.state,
            self.items.get(self.index).unwrap().len(),
        ))
    }

    fn previous(&mut self) {
        self.state.select(get_previous_state_to_select(
            &self.state,
            self.items.get(self.index).unwrap().len(),
        ));
    }
}

impl CommandStates {
    pub fn new(items: Vec<Vec<(String, String)>>) -> Self {
        CommandStates {
            items: items.clone(),
            index: 0,
            state: ListState::default(),
            current_selected: false,
        }
    }

    pub fn set_list_position(&mut self, index: usize) {
        self.index = index;
    }
}

pub struct PopupContent<'a> {
    pub message: &'a str,
    pub confirm: &'a str,
}

impl<'a> PopupContent<'a> {
    pub fn new(message: &'a str, confirm: &'a str) -> Self {
        PopupContent { message, confirm }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub folders: StatefulList<String>,
    pub commands: CommandStates,
    pub show_command_confirmation: bool,
    pub confirmation_popup: PopupContent<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        let (folders, commands) = generate_data();
        App {
            title,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            folders: StatefulList::with_items(folders),
            commands: CommandStates::new(commands),
            show_command_confirmation: false,
            confirmation_popup: PopupContent::new(
                "Are you sure you want the selected command ? (Esc to cancel)",
                "Press Enter",
            ),
        }
    }
}
