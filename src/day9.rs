use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};
use leaky_bucket::RateLimiter;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

pub async fn milk(
    milk_bucket: State<Arc<RateLimiter>>,
    headers: HeaderMap,
    body: String,
) -> (StatusCode, String) {
    let has_milk = milk_bucket.try_acquire(1);

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

pub fn create_milk_bucket() -> Arc<RateLimiter> {
    let rate_limiter = RateLimiter::builder()
        .max(5)
        .interval(Duration::from_secs(1))
        .build();
    Arc::new(rate_limiter)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Volume {
    Gallons(f32),
    Liters(f32),
}

impl Volume {
    const GALLON_PER_LITER: f32 = 0.26417206;
    const LITER_PER_GALLON: f32 = 3.785412;

    fn switch_unit(self) -> Self {
        match self {
            Volume::Gallons(gallon) => Volume::Liters(gallon * Volume::LITER_PER_GALLON),
            Volume::Liters(liters) => Volume::Gallons(liters * Volume::GALLON_PER_LITER),
        }
    }
}
