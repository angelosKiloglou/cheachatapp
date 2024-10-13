use crate::chat::db::{add_chat, get_chat, get_chat_overviews};
use crate::chat::models;
use crate::chat::server::ChatServer;
use crate::chat::session::ChatSession;
use crate::users;
use actix::Addr;
use actix_session::Session;
use actix_web::web::Path;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use sqlx::postgres::PgPool;
use std::cmp::{max, min};

/// Service that handles the websocket connections request on the given chat
#[get("/ws/chat/{chat_id}")]
async fn chat_route(
    path: Path<String>,
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
    session: Session,
) -> Result<HttpResponse, Error> {
    // Authenticate the user
    let user_id = users::authenticate_user(session).await?;

    // Parse the chat id from the path parameter
    let chat_id = path.into_inner().parse::<i64>().expect("failed to parse i64");

    // Start the session actor with a temporary session id
    ws::start(
        ChatSession {
            id: 0,
            chat_id,
            sender_id: user_id,
            addr_server: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}


/// Service that handles the init chat request with the provided user.
/// The server looks if there is already a chat between the users. If there isn't, it creates a new one.
#[post("/chats")]
async fn init_chat(request: web::Json<models::ChatRequest>, session: Session, db_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let user_id = users::authenticate_user(session).await?;

    let recipient = users::get_user(db_pool.get_ref(), &request.recipient).await?;
    let recipient_id = recipient.id;

    // Sort the user ids before creating the chat
    let user1_id = min(user_id, recipient_id);
    let user2_id = max(user_id, recipient_id);

    let chat_id =  if let Some(chat) = get_chat(db_pool.get_ref(), (user1_id, user2_id)).await? {
        chat.id
    } else {
        add_chat(db_pool.get_ref(), (user1_id, user2_id)).await?
    };

    Ok(HttpResponse::Ok().json(chat_id))
}

/// Gets all the chats
#[get("/get-chats")]
pub async fn get_chats(session: Session, db_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let user_id = users::authenticate_user(session).await?;

    let chats = get_chat_overviews(db_pool.get_ref(), user_id).await?;

    Ok(HttpResponse::Ok().json(chats))
}
