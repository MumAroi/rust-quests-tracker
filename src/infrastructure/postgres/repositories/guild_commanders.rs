use std::sync::Arc;

use crate::{
    domain::{
        entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity},
        repositories::guild_commanders::GuildCommanderRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct GuildCommanderPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl GuildCommanderPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl GuildCommanderRepository for GuildCommanderPostgres {
    async fn register(
        &self,
        register_guild_commander_entity: RegisterGuildCommanderEntity,
    ) -> Result<i32> {
        todo!()
    }

    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        todo!()
    }
}
