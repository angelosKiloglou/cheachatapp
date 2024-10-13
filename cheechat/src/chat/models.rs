use crate::users::UserInfo;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::types::time::PrimitiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct Chat {
    pub id: i64,
    pub user1_id: i64,
    pub user2_id: i64,
    pub created_at: PrimitiveDateTime,
    pub last_message_at: Option<PrimitiveDateTime>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub message: String,
    pub created_at: PrimitiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatRequest {
    pub recipient: String,
}

#[derive(Serialize, Debug)]
pub struct ChatOverview {
    pub chat_id: i64,
    pub last_message: Option<String>,
    pub last_message_at: Option<i64>,
    pub other_user: UserInfo
}

