struct Choose<'a, T: 'a> {
    items: Vec<&'a T>,
    offset: usize,
}

impl<'a, T> Choose<'a, T> {
    pub fn new(items: Vec<&'a T>) -> Self {
        Choose { items, offset: 0 }
    }
}

#[derive(Clone, PartialEq)]
pub enum AddType {
    Command,
    Namespace,
}

pub struct Add<'a, T: 'a> {
    pub add_type: Option<AddType>,
    pub items: Vec<&'a T>,
}

impl<'a, T> Add<'a, T> {
    pub fn new(items: Vec<&'a T>) -> Add<'a, T> {
        Add {
            add_type: None,
            items,
        }
    }
}
