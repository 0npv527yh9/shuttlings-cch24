mod entity;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use entity::{Claims, Key};
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, Header, Validation};
use serde_json::Value;
use std::{
    fs,
    sync::{Arc, Mutex},
};

pub async fn wrap(State(key): State<Arc<Mutex<Key>>>, body: String) -> impl IntoResponse {
    let header = Header::default();
    let claims = Claims { gift: body };
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
    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.required_spec_claims.remove("exp");

    let gift = jsonwebtoken::decode::<Claims>(jwt, key, &validation)
        .unwrap()
        .claims
        .gift;
    Ok(gift)
}

pub async fn decode(
    State(key): State<Arc<Mutex<DecodingKey>>>,
    body: String,
) -> Result<String, StatusCode> {
    let key = &key.lock().unwrap();
    let mut validation = Validation::default();
    validation.algorithms = vec![Algorithm::RS256, Algorithm::RS512];
    validation.validate_exp = false;
    validation.required_spec_claims.remove("exp");

    match jsonwebtoken::decode::<Value>(&body, key, &validation) {
        Ok(token_data) => Ok(token_data.claims.to_string()),
        Err(error) => match error.kind() {
            ErrorKind::InvalidSignature => Err(StatusCode::UNAUTHORIZED),
            _ => Err(StatusCode::BAD_REQUEST),
        },
    }
}

pub fn create_key() -> Key {
    "Secret-Key-for-Sign".into()
}

pub fn load_santa_public_key() -> DecodingKey {
    let public_key = fs::read_to_string("src/day16/day16_santa_public_key.pem").unwrap();
    DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap()
}
