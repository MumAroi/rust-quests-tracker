use std::sync::Arc;

use crate::domain::entities::guild_commanders::GuildCommanderEntity;
use crate::domain::models::guild_commander::RegisterGuildCommanderModel;
use crate::domain::repositories::guild_commander::GuildCommanderRepository;
use anyhow::Result;

pub struct GuildCommandersUseCase<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    guild_commander_repository: Arc<T>,
}

impl<T> GuildCommandersUseCase<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    pub fn new(guild_commander_repository: Arc<T>) -> Self {
        Self {
            guild_commander_repository,
        }
    }

    pub async fn register(
        &self,
        register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        todo!()
    }
}
