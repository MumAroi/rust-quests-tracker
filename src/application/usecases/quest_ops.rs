use std::sync::Arc;

use crate::domain::value_objects::quest_model::AddQuestModel;
use crate::domain::value_objects::quest_model::EditQuestModel;
use crate::domain::repositories::quest_ops::QuestOpsRepository;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use anyhow::Result;

pub struct QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    quest_ops_repository: Arc<T1>,
    journey_ledger_repository: Arc<T2>,
}

impl<T1, T2> QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T1>, journey_ledger_repository: Arc<T2>) -> Self {
        Self {
            quest_ops_repository,
            journey_ledger_repository,
        }
    }

    pub async fn add(
        &self,
        guild_commander_id: i32,
        add_quest_model: AddQuestModel,
    ) -> Result<i32> {
        todo!()
    }

    pub async fn edit(
        &self,
        quest_id: i32,
        guild_commander_id: i32,
        edit_quest_model: EditQuestModel,
    ) -> Result<i32> {
        todo!()
    }

    pub async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        todo!()
    }
}
