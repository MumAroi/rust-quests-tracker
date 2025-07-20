use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchboardRepository,
        value_objects::quest_adventurer_junction::QuestAdventurerJunctionModel,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CrewSwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchboardRepository for CrewSwitchboardPostgres {
    async fn join(&self, junction_body_id: QuestAdventurerJunctionModel) -> Result<()> {
        todo!()
    }

    async fn leave(&self, junction_body_id: QuestAdventurerJunctionModel) -> Result<()> {
        todo!()
    }
}
