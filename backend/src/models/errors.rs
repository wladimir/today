#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DatabaseError {
    #[from]
    source: anyhow::Error,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to process password.")]
pub struct PasswordError {
    #[from]
    source: bcrypt::BcryptError,
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserError {
    #[error("There is no user with id {user_id:?}.")]
    NotFound {
        user_id: u32,
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("There is no user with the email and password you specified")]
    NotFound,
    #[error("Failed to process password")]
    PasswordError(#[from] PasswordError),
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}
