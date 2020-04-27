#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: u32,
    pub content: String,
    pub completed: bool,
}

impl Task {}
