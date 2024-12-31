pub mod domain;
mod scheme;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use domain::{
    model::{Quote, QuoteList},
    request::{InsertQuote, Token, UpdateQuote},
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
pub use scheme::create_tables;
use sqlx::PgPool;
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex},
};
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

pub struct ListState {
    pub pool: PgPool,
    page_map: Mutex<HashMap<String, i32>>,
}

pub fn create_list_state(pool: PgPool) -> ListState {
    ListState {
        pool,
        page_map: Mutex::new(HashMap::new()),
    }
}

pub async fn list(
    Query(token): Query<Token>,
    State(state): State<Arc<ListState>>,
) -> Result<String, StatusCode> {
    let page = match token.token {
        Some(token) => {
            let page_map = state.page_map.lock().unwrap();
            *page_map.get(&token).ok_or(StatusCode::BAD_REQUEST)?
        }
        None => 0,
    };

    let mut quotes = sqlx::query_as::<_, Quote>(
        r#"SELECT * FROM quotes
        ORDER BY created_at ASC
        LIMIT 4 OFFSET $1 * 3;"#,
    )
    .bind(page)
    .fetch_all(&state.pool)
    .await
    .unwrap();

    let next_token = (quotes.len() == 4).then(|| {
        quotes.pop();

        let next_token = generate_random_string();
        let next_page = page + 1;
        let mut page_map = state.page_map.lock().unwrap();
        page_map.insert(next_token.clone(), next_page);

        next_token
    });

    Ok(serde_json::to_string(&QuoteList {
        quotes,
        page: page + 1,
        next_token,
    })
    .unwrap())
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

fn generate_random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
