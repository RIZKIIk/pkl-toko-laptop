use crate::{error::AppError, models::LaptopInput, presenters::LaptopResponse, repository};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

pub async fn list(State(pool): State<PgPool>) -> Result<Json<Value>, AppError> {
    let rows = repository::list_laptops(&pool).await?;
    let data: Vec<Value> = rows
        .into_iter()
        .map(|laptop| LaptopResponse::from(laptop).to_array())
        .collect();
    Ok(Json(json!({"status": "success", "data": data})))
}

pub async fn get(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<LaptopResponse>, AppError> {
    repository::get_laptop(&pool, id)
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
        Json(repository::create_laptop(&pool, &input).await?.into()),
    ))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(input): Json<LaptopInput>,
) -> Result<Json<LaptopResponse>, AppError> {
    input.validate().map_err(AppError::InvalidInput)?;
    repository::update_laptop(&pool, id, &input)
        .await?
        .map(|laptop| Json(laptop.into()))
        .ok_or(AppError::NotFound)
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    if repository::delete_laptop(&pool, id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
