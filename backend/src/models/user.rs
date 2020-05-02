#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub password: Password,
}
