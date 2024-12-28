use axum::http::{header, HeaderName, StatusCode};

pub async fn hello_world() -> &'static str {
    "Hello, bird!"
}

pub async fn seek() -> (StatusCode, [(HeaderName, &'static str); 1]) {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
}
