use crate::app::app::StatefulList;

#[derive(Clone, PartialEq)]
pub enum AddType {
    Command,
    Namespace,
}

pub struct Add<'a, T: 'a> {
    pub add_type: Option<AddType>,
    pub state: StatefulList<&'a T>,
}

impl<'a, T> Add<'a, T> {
    pub fn new(items: Vec<&'a T>) -> Add<'a, T> {
        Add {
            add_type: None,
            state: StatefulList::with_items(items),
        }
    }
}
