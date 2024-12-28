use axum::{
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};

mod day2;
mod day5;
mod day9;
mod day_1;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let milk_bucket = Arc::new(Mutex::new(day9::create_milk_bucket()));

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
        .with_state(milk_bucket);

    Ok(router.into())
}
