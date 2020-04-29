use crate::errors::AddTaskError;
use crate::models::{Task, User};

pub trait Repository {
    fn add_task(
        &self,
        user_id: u32,
        task: &Task,
    ) -> Result<Task, AddTaskErrorError>;
}
