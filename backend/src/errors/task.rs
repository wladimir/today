use crate::errors::{DatabaseError, GetProjectError, GetUserError};

#[derive(thiserror::Error, Debug)]
pub enum AddTaskError {
    #[error("There is no user with user id {user_id:?}.")]
    UserNotFound {
        user_id: u32,
        #[source]
        source: GetUserError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}
