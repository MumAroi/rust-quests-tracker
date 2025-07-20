use std::sync::Arc;

use crate::domain::repositories::journey_ledger::JourneyLedgerRepository;
use crate::domain::repositories::quest_ops::QuestOpsRepository;
use anyhow::Result;

pub struct JourneyLedgerUseCase<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestOpsRepository + Send + Sync,
{
    journey_ledger_repository: Arc<T1>,
    quest_ops_repository: Arc<T2>,
}

impl<T1, T2> JourneyLedgerUseCase<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestOpsRepository + Send + Sync,
{
    pub fn new(journey_ledger_repository: Arc<T1>, quest_ops_repository: Arc<T2>) -> Self {
        Self {
            journey_ledger_repository,
            quest_ops_repository,
        }
    }

    pub async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        todo!()
    }

    pub async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        todo!()
    }

    pub async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        todo!()
    }
}
