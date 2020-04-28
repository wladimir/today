use crate::models::Task;

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Project {}
