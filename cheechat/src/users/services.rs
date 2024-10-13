use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};
use bcrypt::{hash, verify};
use sqlx::postgres::PgPool;

use crate::errors::ApiError;
use crate::users::models::{GetUser, UserInfo};

use super::{authenticate_user, db};
use super::{models, RegisterUser};

/// Retrieves all the users.
#[get("/get-users")]
pub async fn get_users(db_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let users = db::get_users(db_pool.get_ref()).await?;

    let users: Vec<_> = users.into_iter().map(|user| UserInfo::from_user(user)).collect();
    Ok(HttpResponse::Ok().json(users))
}

/// Registers a new user.
#[post("/register")]
pub async fn register(
    user: web::Json<RegisterUser>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {

    let mut user: RegisterUser = user.into_inner();
    user.password = hash(user.password, 10).unwrap();

    let _ = db::add_user(db_pool.get_ref(), user).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Handles login by adding a new session based on the user id if the credentials are correct.
#[post("/login")]
pub async fn login(db_pool: web::Data<PgPool>, credentials: web::Json<models::Credentials>, session: Session) -> Result<HttpResponse, Error> {
    let credentials = credentials.into_inner();

    let user = db::get_user(db_pool.get_ref(), &credentials.username).await?;

    if !verify(credentials.password, &user.password).unwrap() {
        return Err(ApiError::AuthError.into());
    }

    let id = user.id.to_string();
    session.insert("user_id", &id)?;
    session.renew();

    Ok(HttpResponse::Ok().finish())
}


/// Handles logout
#[post("/logout")]
pub async fn logout(session: Session) -> actix_web::Result<String> {
    let id: Option<String> = session.get("user_id")?;
    if let Some(x) = id {
        session.purge();
        Ok(format!("Logged out: {x}"))
    } else {
        Ok("Could not log out anonymous user".into())
    }
}

/// Gets a user with details based on the provided username
#[get("/get-user")]
pub async fn get_user(db_pool: web::Data<PgPool>, query_params: web::Query<GetUser>, session: Session) -> Result<HttpResponse, Error> {
    let _ = authenticate_user(session).await?;

    let username = query_params.username.clone();
    println!("user name to search {:}", username);
    let user = db::get_user(db_pool.get_ref(), username.as_str()).await?;

    Ok(HttpResponse::Ok().json(UserInfo::from_user(user)))
}

/// Gets current user
#[get("/get-current-user")]
pub async fn get_current_user_id(session: Session) -> Result<HttpResponse, Error>{
    let user_id = authenticate_user(session).await?;

    Ok(HttpResponse::Ok().json(user_id))
}



