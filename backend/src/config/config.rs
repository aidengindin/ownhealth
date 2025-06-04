use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .build()?;

        settings.try_deserialize()
    }
}

