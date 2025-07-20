use std::sync::Arc;

use crate::domain::repositories::adventurer::AdventurerRepository;
use crate::domain::repositories::guild_commander::GuildCommanderRepository;
use anyhow::Result;

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    adventurers_repository: Arc<T1>,
    guild_commander_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurerRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T1>, guild_commander_repository: Arc<T2>) -> Self {
        Self {
            adventurers_repository,
            guild_commander_repository,
        }
    }

    pub async fn adventurer_login(&self) {
        todo!()
    }

    pub async fn adventurer_refresh_token(&self) {
        todo!()
    }

    pub async fn guild_commander_login(&self) {
        todo!()
    }

    pub async fn guild_commander_refresh_token(&self) {
        todo!()
    }
}
