use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

pub struct Key {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl From<&str> for Key {
    fn from(secret: &str) -> Self {
        let secret = secret.as_bytes();

        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub gift: String,
}
