use crate::core::config::Config;
use tui::widgets::ListState;

pub trait State {
    fn next(&mut self);
    fn previous(&mut self);
}

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
    pub is_selected: bool,
}

impl State for TabsState {
    fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        self.index = (self.index + self.titles.len() - 1) % self.titles.len();
    }
}

impl TabsState {
    pub fn new(config: &Config) -> Self {
        let titles = config
            .name_config
            .tabs_title
            .iter()
            .map(|tab| tab.clone())
            .collect();

        TabsState {
            titles,
            index: 0,
            is_selected: false,
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub is_selected: bool,
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
    pub fn with_items(items: Vec<T>) -> Self {
        StatefulList {
            state: ListState::default(),
            items,
            is_selected: false,
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

    pub fn current_item(&self) -> &T {
        &self.items[self.current()]
    }
}
