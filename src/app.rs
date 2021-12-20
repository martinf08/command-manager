use crate::db::{get_commands_and_tags, get_folders};
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
    pub commands: StatefulList<String>,
    pub show_command_confirmation: bool,
    pub confirmation_popup: PopupContent<'a>,
    pub tags: StatefulList<String>,
    pub quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        let folders = get_folders().expect("Failed to get folders");
        let (commands, tags) =
            get_commands_and_tags(None).expect("Failed to get commands and tags");
        App {
            title,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            folders: StatefulList::with_items(folders),
            commands: StatefulList::with_items(commands),
            show_command_confirmation: false,
            confirmation_popup: PopupContent::new(
                "Are you sure you want the selected command ? (Esc to cancel)",
                "Press Enter",
            ),
            tags: StatefulList::with_items(tags),
            quit: false,
        }
    }
    pub fn set_commands_tags_from_position(&mut self, index: usize) {
        let folder = self.folders.items[index].clone();
        let (commands, tags) =
            get_commands_and_tags(Some(folder)).expect("Failed to get commands and tags");
        self.commands = StatefulList::with_items(commands);
        self.tags = StatefulList::with_items(tags);
    }

    pub fn switch_selected_widgets_off(&mut self) {
        self.folders.current_selected = false;
        self.commands.current_selected = false;
        self.tags.current_selected = false;
    }

    pub fn switch_selected_commands_tags_on(&mut self) {
        self.commands.current_selected = true;
        self.tags.current_selected = true;

        self.commands.state.select(Some(0));
        self.tags.state.select(Some(0));
    }

    pub fn switch_selected_commands_tags_off(&mut self) {
        self.commands.current_selected = false;
        self.tags.current_selected = false;

        self.commands.unselect();
        self.tags.unselect();
    }

    pub fn set_current_selected_commands_tags(&mut self, value: bool) {
        self.commands.current_selected = value;
        self.tags.current_selected = value;
    }

    pub fn switch_selected_folders_on(&mut self) {
        self.folders.current_selected = true;
        self.folders.state.select(Some(0));
        self.set_commands_tags_from_position(self.folders.current());
    }

    pub fn switch_selected_folders_off(&mut self) {
        self.folders.current_selected = false;
        self.folders.unselect();
    }

    pub fn set_current_selected_folder(&mut self, value: bool) {
        self.folders.current_selected = value;
    }

    pub fn set_current_selected_tab(&mut self, value: bool) {
        self.tabs.current_selected = value;
    }

    pub fn set_show_command_confirmation(&mut self, value: bool) {
        self.show_command_confirmation = value;
    }
}
