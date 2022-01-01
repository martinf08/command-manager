use crate::app::app::StatefulList;

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

    pub fn set_add_type(&mut self, add_type: AddType) {
        self.add_type = Some(add_type);
    }
}