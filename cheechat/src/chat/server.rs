use crate::chat::db::{add_message, get_recent_messages};
use actix::{Actor, Context, Handler, Message, Recipient};
use chrono::{Utc};
use rand::prelude::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// The message that gets forwarded from the server to the active sessions.
#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct FwdMessage {
    pub message: String,
    pub sender_id: i64,
    pub sent_at: i64,
}

/// The connect request to the chat, from the chat session to the server.
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    // The address of the session actor, so it can receive the server messages.
    pub addr: Recipient<FwdMessage>,
    pub chat_id: i64
}

/// The disconnect request to the chat, from the chat session to the server.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub chat_id: i64,
}

/// The chat message from the chat session to the server.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub session_id: usize,
    pub content: String,
    pub chat_id: i64,
    pub sender_id: i64,
}

pub struct ChatServer {
    /// Storage of the active sessions and their addresses.
    sessions: HashMap<usize, Recipient<FwdMessage>>,
    /// Storage of the active chats
    chats: HashMap<i64, HashSet<usize>>,
    /// Thread safe random generator to generate session ids upon connection
    rng: ThreadRng,
    /// Thread safe database connection pool reference to store chat messages
    db_pool: Arc<PgPool>,
}

impl ChatServer {
    pub fn new(db_pool: PgPool) -> Self {
        let db_pool = Arc::new(db_pool);
        ChatServer {
            sessions: HashMap::new(),
            chats: HashMap::new(),
            rng: rand::thread_rng(),
            db_pool,
        }
    }

    /// Broadcasts the client message to all the chat sessions that participate on the chat.
    fn broadcast_message(&self, message: &ClientMessage) {
        if let Some(sessions) = self.chats.get(&message.chat_id) {
            for id in sessions {
                if *id == message.session_id {
                    continue;
                }
                if let Some(addr) = self.sessions.get(id) {
                    let _ = addr.do_send(FwdMessage {
                        message: message.content.clone(),
                        sender_id: message.sender_id,
                        sent_at: Utc::now().timestamp(),
                    });
                }
            }
        }
    }
}

impl Actor for ChatServer {
    /// The chat server actor lives in its own context
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    /// The server handles the connection requests from the client as follows:
    ///
    /// - Generates the session id and registers it
    /// - Registers the chat id if not present
    /// - Receives the recent chat messages and send them to the session concurrently
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        // Register the session (client) to the server
        let session_id = self.rng.gen::<usize>();
        self.sessions.insert(session_id, msg.addr.clone());

        self.chats.entry(msg.chat_id).or_default().insert(session_id);

        // Retrieve the chat history and send it back to the client concurrently
        let db_pool = self.db_pool.clone();
        actix::spawn(async move {
            let chat_messages = if let Ok(messages) = get_recent_messages(&db_pool, msg.chat_id, 50).await {
                messages
            } else {
                log::error!("Error retrieving the chat history for {:}", msg.chat_id);
                Vec::new()
            };

            // Send the retrieved messages back to the session
            chat_messages.into_iter()
                .for_each(|chat_message| msg.addr.do_send(FwdMessage {
                    message: chat_message.message,
                    sender_id: chat_message.sender_id,
                    sent_at: chat_message.created_at.assume_utc().unix_timestamp()
                }));
        });

        session_id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    /// The server handles the disconnection requests from the client as follows:
    ///
    /// - Unregisters the session and the chat if it is inactive (no active sessions)
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        // Remove the session from the server
        if self.sessions.remove(&msg.id).is_some() {
            // Determine if the room has no active sessions and it is for removal
            self.chats.entry(msg.chat_id.clone()).and_modify(|set| {
                set.remove(&msg.id);
            });
            if let Some(set) = self.chats.get(&msg.chat_id) {
                if set.is_empty() {
                    self.chats.remove(&msg.chat_id);
                }
            }
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    /// The server handles the connection requests from the client as follows:
    ///
    /// - Broadcasts the message to the active sessions of the chat
    /// - Saves the message in the database concurrently
    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        self.broadcast_message(&msg);

        let db_pool = self.db_pool.clone();
        actix::spawn(async move {
            match add_message(&db_pool, msg.chat_id, msg.sender_id, msg.content.as_str()).await {
                Err(_) => log::error!("Error adding chat message for {}", msg.chat_id),
                Ok(_) => log::debug!("Message {:} is sent to chat {}", msg.content, msg.chat_id)
            }
        });
    }
}