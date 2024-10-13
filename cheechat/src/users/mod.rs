use actix_session::Session;
use actix_web::{web, Error};

mod services;
mod db;
mod models;

pub use db::*;
pub use models::*;
use crate::errors::ApiError;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services::register);
    cfg.service(services::login);
    cfg.service(services::logout);
    cfg.service(services::get_user);
    cfg.service(services::get_current_user_id);
}


/// Authenticates the user.
pub async fn authenticate_user(session: Session) -> Result<i64, Error>{
    let user_id: Option<String> = session.get::<String>("user_id")?;
    if let None = user_id {
        return Err(ApiError::AuthError.into())
    }
    let user_id = user_id.unwrap().parse::<i64>().expect("Error parsing i64 user id");

    Ok(user_id)
}