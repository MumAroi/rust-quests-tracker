use super::{
    config_model::{AdventurersSecret, Database, DotEnvyConfig, GuildCommandersSecret, Server},
    stage::Stage,
};
use anyhow::{Ok, Result};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_srt = std::env::var("STAGE").unwrap_or_default();

    Stage::try_from(&stage_srt).unwrap_or_default()
}

pub fn get_adventurer_secret_env() -> Result<AdventurersSecret> {
    dotenvy::dotenv().ok();

    Ok(AdventurersSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET").unwrap_or_default(),
        refresh_secret: std::env::var("JWT_ADVENTURER_REFRESH_SECRET").unwrap_or_default(),
    })
}

pub fn get_guild_commanders_secret_env() -> Result<GuildCommandersSecret> {
    dotenvy::dotenv().ok();

    Ok(GuildCommandersSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET").unwrap_or_default(),
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET").unwrap_or_default(),
    })
}
