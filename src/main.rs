use axum::{
    routing::{get, post},
    Router,
};

mod day2;
mod day5;
mod day9;
mod day_1;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_1::hello_world))
        .route("/-1/seek", get(day_1::seek))
        .route("/2/dest", get(day2::task1::dest))
        .route("/2/key", get(day2::task2::key))
        .route("/2/v6/dest", get(day2::task3::dest))
        .route("/2/v6/key", get(day2::task3::key))
        .route("/5/manifest", post(day5::manifest))
        .route("/9/milk", post(day9::milk))
        .with_state(day9::create_milk_bucket());

    Ok(router.into())
}
