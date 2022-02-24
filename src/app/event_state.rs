#[derive(Debug, PartialEq)]
pub enum Mode {
    Add,
    Delete,
    Normal,
}

pub enum SubMode {
    Command,
    Namespace,
    None,
    Quit,
}

pub enum EventType {
    Command,
    Namespace,
    None,
    Tag,
}

pub enum Confirm {
    Confirmed,
    Display(String),
    Hide,
}

pub enum Tab {
    Tab1,
    Tab2,
    Tab3,
}

pub struct EventState {
    pub confirm: Confirm,
    pub event_type: EventType,
    pub mode: Mode,
    pub sub_mode: SubMode,
    pub tab: Tab,
}

impl EventState {
    pub fn new() -> Self {
        EventState {
            confirm: Confirm::Hide,
            event_type: EventType::None,
            mode: Mode::Normal,
            sub_mode: SubMode::None,
            tab: Tab::Tab1,
        }
    }
}
