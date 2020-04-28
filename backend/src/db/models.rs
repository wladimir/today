use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}
