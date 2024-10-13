use crate::app::config::AppConfig;
use crate::users::authenticate_user;
use crate::{chat, users};
use actix::Actor;
use actix_cors::Cors;
use actix_session::storage::RedisSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

pub mod config;

/// Initiates the server and run it.
pub async fn run(config: AppConfig) -> std::io::Result<()> {
    // Create the postgres db pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await.unwrap();

    // Create the redis session store
    let private_key = actix_web::cookie::Key::generate();
    let store = RedisSessionStore::new(config.res_addr)
        .await
        .unwrap();

    // Create the chat server actor
    let chat_server = chat::ChatServer::new(db_pool.clone()).start();

    // Load t
    let tls_config = config::load_tls_config();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) // Permissive cors for simplicity
            .wrap(SessionMiddleware::builder(store.clone(), private_key.clone()).build())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .configure(users::init_routes)
            .configure(chat::init_routes)
            .service(index)
    })
        .bind_rustls_0_23(config.server_addr.clone(), tls_config)?
        .run();
    println!("Server running at https://{}/", config.server_addr);

    server.await
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct IndexResponse {
    user_id: i64,
}

#[get("/")]
pub async fn index(session: Session) -> actix_web::Result<HttpResponse, Error> {
    let user_id = authenticate_user(session).await?;
    Ok(HttpResponse::Ok().json(IndexResponse { user_id}))
}


