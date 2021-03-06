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
}

#[derive(PartialEq)]
pub enum EventType {
    Command,
    Namespace,
    None,
    Tag,
}

#[derive(PartialEq)]
pub enum Confirm {
    Confirmed,
    Display,
    Hide,
}

pub enum Tab {
    Tab1,
}

pub struct EventState {
    confirm: Confirm,
    event_type: EventType,
    mode: Mode,
    sub_mode: SubMode,
    tab: Tab,
}

impl Default for EventState {
    fn default() -> Self {
        EventState {
            confirm: Confirm::Hide,
            event_type: EventType::None,
            mode: Mode::Normal,
            sub_mode: SubMode::None,
            tab: Tab::Tab1,
        }
    }
}

impl EventState {
    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn get_sub_mode(&self) -> &SubMode {
        &self.sub_mode
    }

    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn get_confirm(&self) -> &Confirm {
        &self.confirm
    }

    pub fn get_tab(&self) -> &Tab {
        &self.tab
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_sub_mode(&mut self, sub_mode: SubMode) {
        self.sub_mode = sub_mode;
    }

    pub fn set_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
    }

    pub fn set_confirm(&mut self, confirm: Confirm) {
        self.confirm = confirm;
    }
}
