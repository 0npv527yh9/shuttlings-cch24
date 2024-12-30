mod entity;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use entity::{Claims, Key};
use jsonwebtoken::{Header, Validation};
use std::sync::{Arc, Mutex};

pub async fn wrap(State(key): State<Arc<Mutex<Key>>>, body: String) -> impl IntoResponse {
    let header = Header::default();
    let claims = Claims::new(&body);
    let key = &key.lock().unwrap().encoding_key;
    let jwt = jsonwebtoken::encode(&header, &claims, key).unwrap();

    CookieJar::new().add(Cookie::new("gift", jwt))
}

pub async fn unwrap(
    cookie_jar: CookieJar,
    State(key): State<Arc<Mutex<Key>>>,
) -> Result<String, StatusCode> {
    let jwt = cookie_jar
        .get("gift")
        .ok_or(StatusCode::BAD_REQUEST)?
        .value();
    let key = &key.lock().unwrap().decoding_key;
    let validation = &Validation::default();

    let gift = jsonwebtoken::decode::<Claims>(jwt, key, validation)
        .unwrap()
        .claims
        .gift;
    Ok(gift)
}

pub fn create_key() -> Key {
    "Secret-Key-for-Sign".into()
}
