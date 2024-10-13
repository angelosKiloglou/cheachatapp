use crate::chat::models::{Chat, ChatMessage, ChatOverview};
use crate::errors::ApiError;
use crate::users::UserInfo;
use sqlx::postgres::PgPool;

/// Retrieves the chat for the provided users
pub async fn get_chat(pool: &PgPool, (user1_id, user2_id): (i64, i64)) -> Result<Option<Chat>, ApiError> {
    assert!(user1_id < user2_id);

    let chat = sqlx::query_as!(Chat, "SELECT * FROM cheechat.chats WHERE user1_id=$1 AND user2_id=$2", user1_id, user2_id)
        .fetch_optional(pool)
        .await?;

    Ok(chat)
}

/// Adds the chat for the provided users
pub async fn add_chat(pool: &PgPool, (user1_id, user2_id): (i64, i64)) -> Result<i64, ApiError> {
    assert!(user1_id < user2_id);

    let row = sqlx::query!(
        "INSERT INTO cheechat.chats (user1_id, user2_id) VALUES ($1, $2) RETURNING  id", user1_id, user2_id)
        .fetch_one(pool)
        .await?;

    Ok(row.id)
}

/// Get the recent messages of the chat
pub async fn get_recent_messages(pool: &PgPool, chat_id: i64, limit: i64) -> Result<Vec<ChatMessage>, ApiError> {
    let messages = sqlx::query_as!(
        ChatMessage,
        r#"
        SELECT *
FROM (
    SELECT *
    FROM cheechat.chat_messages
    WHERE chat_id = $1
    ORDER BY created_at DESC
    LIMIT $2
) AS latest_messages
ORDER BY created_at ASC;
        "#,
        chat_id,
        limit
    )
        .fetch_all(pool)
        .await?;

    Ok(messages)
}

/// Adds chat message to the chat
pub async fn add_message(pool: &PgPool, chat_id: i64, sender_id: i64, message: &str) -> Result<i64, ApiError> {
    let row = sqlx::query!(
        "INSERT INTO cheechat.chat_messages (chat_id, sender_id, message) VALUES ($1, $2, $3) RETURNING id",
        chat_id,
        sender_id,
        message
    )
        .fetch_one(pool)
        .await?;

    let _update_chat = update_chat_last_message(pool, chat_id).await?;

    Ok(row.id)
}

/// Updates the chat when a new chat message is added
async fn update_chat_last_message(pool: &PgPool, chat_id: i64) -> Result<(), ApiError> {
    let _query = sqlx::query!("UPDATE cheechat.chats SET last_message_at = CURRENT_TIMESTAMP WHERE id = $1", chat_id)
        .fetch_one(pool)
        .await?;

    Ok(())
}

/// Retrieves the chat overviews for the given user
pub async fn get_chat_overviews(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<ChatOverview>, ApiError> {
    let chat_overviews = sqlx::query!(
        r#"
        SELECT
            chats.id AS chat_id,
            chat_messages.message AS "last_message: Option<String>",
            chat_messages.created_at AS "last_message_at: Option<sqlx::types::time::PrimitiveDateTime>",
            CASE
                WHEN chats.user1_id = $1 THEN users2.id
                ELSE users1.id
            END AS other_user_id,
            CASE
                WHEN chats.user1_id = $1 THEN users2.email
                ELSE users1.email
            END AS other_user_email,
            CASE
                WHEN chats.user1_id = $1 THEN users2.first_name
                ELSE users1.first_name
            END AS other_user_first_name,
            CASE
                WHEN chats.user1_id = $1 THEN users2.last_name
                ELSE users1.last_name
            END AS other_user_last_name,
            CASE
                WHEN chats.user1_id = $1 THEN users2.username
                ELSE users1.username
            END AS other_user_username
        FROM cheechat.chats AS chats
        LEFT JOIN LATERAL (
            SELECT message, created_at
            FROM cheechat.chat_messages
            WHERE chat_messages.chat_id = chats.id
            ORDER BY created_at DESC
            LIMIT 1
        ) AS chat_messages ON true
        JOIN cheechat.users AS users1 ON users1.id = chats.user1_id
        JOIN cheechat.users AS users2 ON users2.id = chats.user2_id
        WHERE chats.user1_id = $1 OR chats.user2_id = $1
        ORDER BY last_message_at DESC;
        "#,
        user_id
    )
        .map(|row| ChatOverview {
            chat_id: row.chat_id,
            last_message: row.last_message,
            last_message_at: row.last_message_at.map(|dt| dt.assume_utc().unix_timestamp()),
            other_user: UserInfo {
                id: row.other_user_id.unwrap(),
                email: row.other_user_email.unwrap(),
                first_name: row.other_user_first_name.unwrap(),
                last_name: row.other_user_last_name.unwrap(),
                username: row.other_user_username.unwrap(),
            },
        })
        .fetch_all(pool)
        .await?;

    println!("These are the chats: {:?}", &chat_overviews);
    Ok(chat_overviews)
}

