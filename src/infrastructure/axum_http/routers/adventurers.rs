use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use std::sync::Arc;

use crate::{
    application::usecases::adventurers::AdventurerUseCase,
    domain::{
        repositories::adventurers::AdventurerRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::adventurers::AdventurerPostgres,
    },
};

pub fn routers(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurerPostgres::new(db_pool);
    let adventurers_use_case = AdventurerUseCase::new(Arc::new(adventurers_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurers_use_case))
}

pub async fn register<T>(
    State(adventurers_use_case): State<Arc<AdventurerUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: AdventurerRepository + Send + Sync,
{
}
