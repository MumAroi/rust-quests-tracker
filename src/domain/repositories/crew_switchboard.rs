use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::value_objects::quest_adventurer_junction::QuestAdventurerJunctionModel;

#[async_trait]
#[automock]
pub trait CrewSwitchboardRepository {
    async fn join(&self, junction_body_id: QuestAdventurerJunctionModel) -> Result<()>;
    async fn leave(&self, junction_body_id: QuestAdventurerJunctionModel) -> Result<()>;
}
