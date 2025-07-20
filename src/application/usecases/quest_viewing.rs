use std::sync::Arc;

use crate::domain::models::board_checking::BoardCheckingFilter;
use crate::domain::models::quest::QuestModel;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use anyhow::Result;

pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        todo!()
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        todo!()
    }
}
