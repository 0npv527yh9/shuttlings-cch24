use axum::{extract::State, http::StatusCode};
use leaky_bucket::RateLimiter;
use std::{sync::Arc, time::Duration};

pub async fn milk(milk_bucket: State<Arc<RateLimiter>>) -> (StatusCode, String) {
    let has_milk = milk_bucket.try_acquire(1);

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
