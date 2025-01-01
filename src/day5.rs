use std::fmt::Display;

use axum::http::{HeaderMap, StatusCode};
use cargo_manifest::{Manifest, Package};
use itertools::Itertools;
use toml::Value;

struct Order {
    item: String,
    quantity: u32,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.item, self.quantity))
    }
}

pub async fn manifest(
    headers: HeaderMap,
    body: String,
) -> Result<String, (StatusCode, &'static str)> {
    let content_type = headers
        .get("Content-Type")
        .and_then(|content_type| content_type.to_str().ok());

    let manifest: Manifest = match content_type {
        Some("application/toml") => toml::from_str(&body).ok(),
        Some("application/yaml") => serde_yml::from_str(&body).ok(),
        Some("application/json") => serde_json::from_str(&body).ok(),
        _ => return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, "")),
    }
    .ok_or((StatusCode::BAD_REQUEST, "Invalid manifest"))?;

    let (keywords, metadata) = match manifest.package {
        Some(Package {
            keywords, metadata, ..
        }) => {
            let keywords = keywords.and_then(|keywords| keywords.as_local());
            (keywords, metadata)
        }
        None => (None, None),
    };

    keywords
        .filter(|keywords| keywords.contains(&"Christmas 2024".to_string()))
        .ok_or((StatusCode::BAD_REQUEST, "Magic keyword not provided"))?;

    if let Some(orders) = into_orders(metadata).filter(|orders| !orders.is_empty()) {
        Ok(orders.iter().join("\n"))
    } else {
        Err((StatusCode::NO_CONTENT, ""))
    }
}

fn into_orders(metadata: Option<Value>) -> Option<Vec<Order>> {
    let orders = metadata?
        .get("orders")?
        .as_array()?
        .iter()
        .filter_map(Value::as_table)
        .filter_map(|order| {
            if let (Some(Value::String(item)), Some(Value::Integer(quantity))) =
                (order.get("item"), order.get("quantity"))
            {
                Some(Order {
                    item: item.to_owned(),
                    quantity: *quantity as u32,
                })
            } else {
                None
            }
        })
        .collect();
    Some(orders)
}
