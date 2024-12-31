mod day12;
mod day16;
mod day19;
mod day2;
mod day5;
mod day9;
mod day_1;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let milk_bucket = Arc::new(Mutex::new(day9::create_milk_bucket()));
    let board_state = Arc::new(Mutex::new(day12::create_state()));
    let key = Arc::new(Mutex::new(day16::create_key()));
    let santa_publilc_key = Arc::new(Mutex::new(day16::load_santa_public_key()));
    let pool = Arc::new(day19::migrate(pool).await);

    let router = Router::new()
        .route("/", get(day_1::hello_world))
        .route("/-1/seek", get(day_1::seek))
        .route("/2/dest", get(day2::task1::dest))
        .route("/2/key", get(day2::task2::key))
        .route("/2/v6/dest", get(day2::task3::dest))
        .route("/2/v6/key", get(day2::task3::key))
        .route("/5/manifest", post(day5::manifest))
        .route("/9/milk", post(day9::milk))
        .with_state(milk_bucket.clone())
        .route("/9/refill", post(day9::refill))
        .with_state(milk_bucket)
        .route("/12/board", get(day12::board))
        .with_state(board_state.clone())
        .route("/12/reset", post(day12::reset))
        .with_state(board_state.clone())
        .route("/12/place/:team/:column", post(day12::place))
        .with_state(board_state.clone())
        .route("/12/random-board", get(day12::random_board))
        .with_state(board_state)
        .route("/16/wrap", post(day16::wrap))
        .with_state(key.clone())
        .route("/16/unwrap", get(day16::unwrap))
        .with_state(key)
        .route("/16/decode", post(day16::decode))
        .with_state(santa_publilc_key)
        .route("/19/reset", post(day19::reset))
        .with_state(pool.clone())
        .route("/19/cite/:id", get(day19::cite))
        .with_state(pool.clone())
        .route("/19/remove/:id", delete(day19::remove))
        .with_state(pool.clone())
        .route("/19/undo/:id", put(day19::undo))
        .with_state(pool.clone())
        .route("/19/draft", post(day19::draft))
        .with_state(pool);

    Ok(router.into())
}
