use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use std::sync::Arc;

use crate::{
    application::usecases::guild_commanders::GuildCommandersUseCase,
    domain::{
        repositories::guild_commanders::GuildCommanderRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommanderPostgres,
    },
};

pub fn routers(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commander_repository = GuildCommanderPostgres::new(db_pool);
    let guild_commander_use_case =
        GuildCommandersUseCase::new(Arc::new(guild_commander_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commander_use_case))
}

pub async fn register<T>(
    State(guild_commander_use_case): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommanderRepository + Send + Sync,
{
}
