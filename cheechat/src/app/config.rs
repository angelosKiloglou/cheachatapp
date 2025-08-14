use confik::Configuration;

#[derive(Debug, Default, Configuration)]
pub struct AppConfig {
    pub app_name: String,
    pub server_addr: String,
    pub database_url: String,
    pub redis_addr: String,
}