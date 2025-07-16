use std::sync::Arc;

use crate::{
    domain::{
        repositories::journey_ledger::JourneyLedgerRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct JourneyLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl JourneyLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JourneyLedgerRepository for JourneyLedgerPostgres {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        todo!()
    }

    async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        todo!()
    }

    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        todo!()
    }
}
