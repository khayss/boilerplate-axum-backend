use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserForCreate {}

#[derive(Debug, Deserialize)]
pub struct UserForUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UserForUpdate {}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    date_created: String,
}
impl User {}

#[derive(Debug, Deserialize)]
pub struct UserForLogin {
    pub username: String,
    pub password: String,
}
