use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use leaky_bucket::RateLimiter;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub async fn milk(
    milk_bucket: State<Arc<Mutex<RateLimiter>>>,
    headers: HeaderMap,
    body: String,
) -> (StatusCode, String) {
    let has_milk = milk_bucket.lock().unwrap().try_acquire(1);

    let content_type = headers
        .get("Content-Type")
        .and_then(|content_type| content_type.to_str().ok());

    match content_type {
        Some("application/json") => {
            if let Ok(volume) = serde_json::from_str::<Volume>(&body) {
                let volume = volume.switch_unit();
                (StatusCode::OK, serde_json::to_string(&volume).unwrap())
            } else {
                (StatusCode::BAD_REQUEST, String::new())
            }
        }
        _ => task1(has_milk),
    }
}

pub async fn refill(milk_bucket: State<Arc<Mutex<RateLimiter>>>) -> impl IntoResponse {
    let mut milk_bucket = milk_bucket.lock().unwrap();
    *milk_bucket = create_milk_bucket();
    StatusCode::OK
}

fn task1(has_milk: bool) -> (StatusCode, String) {
    if has_milk {
        (StatusCode::OK, "Milk withdrawn\n".to_string())
    } else {
        (
            StatusCode::TOO_MANY_REQUESTS,
            "No milk available\n".to_string(),
        )
    }
}

pub fn create_milk_bucket() -> RateLimiter {
    RateLimiter::builder()
        .max(5)
        .initial(5)
        .interval(Duration::from_secs(1))
        .build()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Volume {
    Gallons(f32),
    Liters(f32),
    Litres(f32),
    Pints(f32),
}

impl Volume {
    const LITER_PER_GALLON: f32 = 3.785412;
    const LITRES_PER_PINT: f32 = 0.56826127;

    fn switch_unit(self) -> Self {
        match self {
            Volume::Gallons(gallon) => Volume::Liters(gallon * Volume::LITER_PER_GALLON),
            Volume::Liters(liters) => Volume::Gallons(liters / Volume::LITER_PER_GALLON),
            Volume::Litres(litres) => Volume::Pints(litres / Volume::LITRES_PER_PINT),
            Volume::Pints(pints) => Volume::Litres(pints * Volume::LITRES_PER_PINT),
        }
    }
}
