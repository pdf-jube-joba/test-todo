#[derive(Debug, Clone, PartialEq)]
pub struct Todo(pub bool, pub String);

impl Todo {
    fn new(string: String) -> Self {
        Todo(false, string)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub usize);

pub trait Repository {
    fn new() -> Self where Self: Sized;
    fn list(&self) -> Vec<Id>;
    fn add(&mut self, todo: Todo) -> Id;
    fn check(&mut self, id: Id) -> Option<()>;
}
