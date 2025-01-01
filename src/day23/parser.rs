use axum::http::StatusCode;
use axum_extra::extract::Multipart;
use hex::FromHexError;
use toml::{Table, Value};

pub enum ParseError {
    Lockfile,
    Checksum,
}

pub async fn parse_multipart(mut multipart: Multipart, name: &str) -> Result<String, StatusCode> {
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                if let Some(field_name) = field.name() {
                    if field_name == name {
                        if let Ok(text) = field.text().await {
                            return Ok(text);
                        } else {
                            return Err(StatusCode::BAD_REQUEST);
                        }
                    }
                }
            }
            _ => return Err(StatusCode::BAD_REQUEST),
        }
    }
}

pub fn parse_lock_file(lock_file: &str) -> Result<Vec<Value>, ParseError> {
    let mut table = lock_file
        .parse::<Table>()
        .map_err(|_| ParseError::Lockfile)?;

    let packages = table
        .remove("package")
        .and_then(|v| match v {
            Value::Array(vec) => Some(vec),
            _ => None,
        })
        .unwrap_or(vec![]);

    Ok(packages)
}

pub fn extract_checksums(packages: &[Value]) -> Result<Vec<Vec<u8>>, ParseError> {
    let mut checksums = vec![];

    for package in packages {
        match package.get("checksum") {
            Some(Value::String(checksum)) => match parse_hex(checksum) {
                Ok(bytes) => checksums.push(bytes),
                Err(_) => return Err(ParseError::Checksum),
            },
            Some(_) => return Err(ParseError::Lockfile),
            _ => (),
        }
    }

    Ok(checksums)
}

pub fn parse_hex(input: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(input).and_then(|bytes| {
        if bytes.len() >= 5 {
            Ok(bytes)
        } else {
            Err(FromHexError::InvalidStringLength)
        }
    })
}
