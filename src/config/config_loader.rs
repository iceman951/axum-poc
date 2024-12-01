use anyhow::Result;

use super::config_model::{Database, DotEnvyConfig, JwtsSecret, Server};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT must be set")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT must be set")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT must be set")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Ok(DotEnvyConfig { server, database });
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_jwt_secret_env() -> Result<JwtsSecret> {
    dotenvy::dotenv().ok();

    Ok(JwtsSecret {
        secret: std::env::var("JWT_SECRET").unwrap_or("".to_string()),
        refresh_secret: std::env::var("JWT_REFRESH_SECRET")?,
    })
}
