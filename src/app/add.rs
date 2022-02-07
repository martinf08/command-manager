#[derive(Clone, PartialEq)]
pub enum AddType {
    Command,
    Namespace,
}

#[derive(PartialEq)]
pub enum InputMode {
    Command,
    Namespace,
    Tag,
}

pub struct Add<'a, T: 'a> {
    pub add_type: Option<AddType>,
    pub items: Vec<&'a T>,
    pub input_mode: Option<InputMode>,
    pub input: String,
    pub input_command: Option<String>,
    pub error_message: Option<String>,
}

impl<'a, T> Add<'a, T> {
    pub fn new(items: Vec<&'a T>) -> Add<'a, T> {
        Add {
            add_type: None,
            items,
            input_mode: None,
            input: String::new(),
            input_command: None,
            error_message: None,
        }
    }
}
