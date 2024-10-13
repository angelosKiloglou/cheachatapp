use cheechat::{app};
use confik::{Configuration, EnvSource};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().unwrap();

    let config = app::config::AppConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    app::run(config).await?;

    Ok(())
}