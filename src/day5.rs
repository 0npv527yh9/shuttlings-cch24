use std::fmt::Display;

use axum::http::{HeaderMap, StatusCode};
use cargo_manifest::{Manifest, MaybeInherited};
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

    get_keywords(&manifest)
        .filter(|&keywords| keywords.contains(&"Christmas 2024".to_string()))
        .ok_or((StatusCode::BAD_REQUEST, "Magic keyword not provided"))?;

    if let Some(orders) = get_orders(&manifest).filter(|orders| !orders.is_empty()) {
        Ok(orders.iter().join("\n"))
    } else {
        Err((StatusCode::NO_CONTENT, ""))
    }
}

fn get_keywords(manifest: &Manifest) -> Option<&Vec<String>> {
    match manifest.package.as_ref()?.keywords.as_ref()? {
        MaybeInherited::Local(keywords) => Some(keywords),
        _ => None,
    }
}

fn get_orders(manifest: &Manifest) -> Option<Vec<Order>> {
    let orders = manifest
        .package
        .as_ref()?
        .metadata
        .as_ref()?
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
