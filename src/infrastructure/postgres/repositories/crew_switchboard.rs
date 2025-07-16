use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchBoardRepository,
        value_objects::quest_adventurer_junction::QuestAdventurerJunctionModel,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CrewSwitchBoardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchBoardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchBoardRepository for CrewSwitchBoardPostgres {
    async fn join(&self, junction_body_id: QuestAdventurerJunctionModel) -> Result<()> {
        todo!()
    }

    async fn leave(&self, junction_body_id: QuestAdventurerJunctionModel) -> Result<()> {
        todo!()
    }
}
