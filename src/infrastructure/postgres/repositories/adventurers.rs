use crate::{
    domain::{
        entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity},
        repositories::adventurers::AdventurerRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct AdventurerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurerRepository for AdventurerPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        todo!()
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        todo!()
    }
}
