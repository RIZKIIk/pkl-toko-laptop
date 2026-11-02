use crate::{error::AppError, models::LaptopInput, presenters::LaptopResponse, repository};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

pub async fn list(State(pool): State<PgPool>) -> Result<Json<Vec<LaptopResponse>>, AppError> {
    let laptops = repository::list(&pool).await?;
    Ok(Json(laptops.into_iter().map(Into::into).collect()))
}

pub async fn get(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<LaptopResponse>, AppError> {
    repository::find(&pool, id)
        .await?
        .map(|laptop| Json(laptop.into()))
        .ok_or(AppError::NotFound)
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(input): Json<LaptopInput>,
) -> Result<(StatusCode, Json<LaptopResponse>), AppError> {
    input.validate().map_err(AppError::InvalidInput)?;
    Ok((
        StatusCode::CREATED,
        Json(repository::create(&pool, &input).await?.into()),
    ))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(input): Json<LaptopInput>,
) -> Result<Json<LaptopResponse>, AppError> {
    input.validate().map_err(AppError::InvalidInput)?;
    repository::update(&pool, id, &input)
        .await?
        .map(|laptop| Json(laptop.into()))
        .ok_or(AppError::NotFound)
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    if repository::delete(&pool, id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
