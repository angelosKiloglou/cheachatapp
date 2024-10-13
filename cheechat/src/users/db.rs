use crate::errors::ApiError;
use crate::users::models::User;
use crate::users::RegisterUser;
use sqlx::postgres::PgPool;

/// Retrieves all users
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, ApiError> {
    let users = sqlx::query_as!(User, "SELECT * FROM cheechat.users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

/// Retrieves the user based on the username.
pub async fn get_user(pool: &PgPool, username: &str) -> Result<User, ApiError> {
    let user = sqlx::query_as!(User, "SELECT * FROM cheechat.users where username=$1", username)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

/// Adds a new user.
pub async fn add_user(pool: &PgPool, user: RegisterUser) -> Result<i64, ApiError> {
    let row = sqlx::query!(
            "INSERT INTO cheechat.users (username, password, first_name, last_name, email) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            user.username,
            user.password,
            user.email,
            user.first_name,
            user.last_name
        )
        .fetch_one(pool)
        .await?;

    Ok(row.id)
}
