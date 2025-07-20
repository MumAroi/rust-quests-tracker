use std::sync::Arc;

use crate::domain::models::adventurer::RegisterAdventurerModel;
use crate::domain::repositories::adventurer::AdventurerRepository;
use anyhow::Result;

pub struct AdventurerUseCase<T>
where
    T: AdventurerRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurerUseCase<T>
where
    T: AdventurerRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        todo!()
    }
}
