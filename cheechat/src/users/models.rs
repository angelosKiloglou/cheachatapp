use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub username: String,
}

impl UserInfo {
    pub fn from_user(user: User) -> Self{
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetUser {
    pub username: String
}

