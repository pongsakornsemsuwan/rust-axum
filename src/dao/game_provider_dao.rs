use axum::http::StatusCode;
use uuid::Uuid;

pub async fn get_game_provider_by_id(mut conn: PoolConnection<Postgres>, game_provider_id: Uuid) -> Option<GameProvider> {
    match sqlx::query_as::<_, GameProvider>("SELECT * FROM game_provider WHERE id = $1")
    .bind(game_provider_id)
    .fetch_one(&mut conn)
    .await {
        Ok(game_provider) => Some(game_provider),
        Err(err) => None,
    }
}

pub async fn get_active_providers_by_type(mut conn: PoolConnection<Postgres>, provider_type: GameProviderType) -> Result<Vector<GameProviderType>, impl IntoResponse> {
    match sqlx::query_as::<_, GameProvider>("SELECT * FROM game_provider WHERE provider_type = $1")
    .bind(provider_type)
    .fetch_all(&mut conn)
    .await {
        Ok(game_providers) => Ok(game_providers),
        Err(err) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
// pub async fn get_active_game_providers