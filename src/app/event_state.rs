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
    Tag,
    None,
}

pub enum Confirm {
    Confirmed,
    Display(String),
    Hide,
}

pub struct EventState {
    pub mode: Mode,
    pub sub_mode: SubMode,
    pub event_type: EventType,
    pub confirm: Confirm,
}

impl EventState {
    pub fn new() -> Self {
        EventState {
            mode: Mode::Normal,
            sub_mode: SubMode::None,
            event_type: EventType::None,
            confirm: Confirm::Hide,
        }
    }
}
