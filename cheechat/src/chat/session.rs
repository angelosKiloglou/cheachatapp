use crate::chat::server::*;
use actix::{fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};

/// The chat session actor struct.
pub struct ChatSession {
    pub id: usize,
    pub chat_id: i64,
    pub sender_id: i64,
    // The address of the chat server actor, so it can send the chat requests
    pub addr_server: Addr<ChatServer>
}

impl Actor for ChatSession {
    /// The chat session actor lives in the websocket context.
    type Context = ws::WebsocketContext<Self>;

    /// When the actor is created, he sends a connect request to the chat server and wait for response.
    /// Then it replaces the temporary session id with the one created from the server.
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr_server.send(Connect {
            addr: addr.recipient(),
            chat_id: self.chat_id,
        })
            // Chain the session id received with the replacement of the temporary one
            .into_actor(self)
            .then(|res, act,  ctx| {
                match res {
                    Ok(id) => act.id = id,
                   _ => ctx.stop(),
                }
                // Return a ready future with empty value
                fut::ready(())
            })
            .wait(ctx);
    }

    /// When the actor stops, he sends a disconnect request without waiting.
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr_server.do_send(Disconnect { id: self.id, chat_id: self.chat_id });
        Running::Stop
    }
}

impl Handler<FwdMessage> for ChatSession{
    type Result = ();

    /// The actor handles forwarded messages from the server as follows:
    /// - Serialises the message to json and writes it in the websocket.
    fn handle(&mut self, msg: FwdMessage, ctx: &mut Self::Context) -> Self::Result {
        let msg = serde_json::to_string(&msg).unwrap();

        ctx.text(msg);
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for ChatSession {
    /// The actor handles web socket messages as follows:
    /// - Sends the text type messages to the chat server
    /// - Close the web socket connection for the unexpected message types
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg
        };

        match msg {
            Message::Ping(msg) => {
                ctx.pong(&msg);
            }
            Message::Text(msg) => {
                self.addr_server.do_send(ClientMessage {
                    session_id: self.id,
                    content: msg.trim().to_owned(),
                    chat_id: self.chat_id,
                    sender_id: self.sender_id,
                })
            }
            Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            Message::Nop => (),
            Message::Pong(_) => (),
            Message::Continuation(_) => ctx.stop(),
            Message::Binary(_) => {
                log::error!("Unexpected message received, dropping the connection");
                ctx.stop();
            }
        }
    }
}