use actix_web::web;
pub use server::ChatServer;

mod server;
mod session;
mod services;
mod db;
mod models;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services::chat_route);
    cfg.service(services::init_chat);
    cfg.service(services::get_chats);
}