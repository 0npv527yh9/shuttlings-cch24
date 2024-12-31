mod entity;
mod scheme;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{InsertQuote, Quote, UpdateQuote};
pub use scheme::create_tables;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use uuid::Uuid;

pub async fn reset(State(pool): State<Arc<PgPool>>) {
    sqlx::query("TRUNCATE TABLE quotes")
        .execute(pool.deref())
        .await
        .unwrap();
}

pub async fn cite(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<String, StatusCode> {
    sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.deref())
        .await
        .unwrap()
        .ok_or(StatusCode::NOT_FOUND)
        .map(|quote| serde_json::to_string(&quote).unwrap())
}

pub async fn remove(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<String, StatusCode> {
    sqlx::query_as::<_, Quote>(
        r#"DELETE FROM quotes WHERE id = $1
        RETURNING id, author, quote, created_at, version"#,
    )
    .bind(id)
    .fetch_optional(pool.deref())
    .await
    .unwrap()
    .ok_or(StatusCode::NOT_FOUND)
    .map(|quote| serde_json::to_string(&quote).unwrap())
}

pub async fn undo(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(update_quote): Json<UpdateQuote>,
) -> Result<String, StatusCode> {
    sqlx::query_as::<_, Quote>(
        r#"UPDATE quotes SET author = $1, quote = $2, version = version + 1 WHERE id = $3
           RETURNING id, author, quote, created_at, version"#,
    )
    .bind(update_quote.author)
    .bind(update_quote.quote)
    .bind(id)
    .fetch_optional(pool.deref())
    .await
    .unwrap()
    .ok_or(StatusCode::NOT_FOUND)
    .map(|quote| serde_json::to_string(&quote).unwrap())
}

pub async fn draft(
    State(pool): State<Arc<PgPool>>,
    Json(insert_quote): Json<InsertQuote>,
) -> (StatusCode, String) {
    let quote = sqlx::query_as::<_, Quote>(
        r#"INSERT INTO quotes (id, author, quote) VALUES ($1, $2, $3)
        RETURNING id, author, quote, created_at, version"#,
    )
    .bind(Uuid::new_v4())
    .bind(insert_quote.author)
    .bind(insert_quote.quote)
    .fetch_one(pool.deref())
    .await
    .unwrap();

    (StatusCode::CREATED, serde_json::to_string(&quote).unwrap())
}
