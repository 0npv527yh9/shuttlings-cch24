pub mod task1 {

    use std::{fmt::Display, str::FromStr};

    use axum::http::StatusCode;
    use cargo_manifest::{Manifest, MaybeInherited};
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

    pub async fn manifest(body: String) -> (StatusCode, String) {
        let manifest = match Manifest::from_str(&body) {
            Ok(manifest) => manifest,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string()),
        };

        let is_valid_keywords = match get_keywords(&manifest) {
            Some(keywords) => keywords.contains(&"Christmas 2024".to_string()),
            None => false,
        };

        if !is_valid_keywords {
            return (
                StatusCode::BAD_REQUEST,
                "Magic keyword not provided".to_string(),
            );
        }

        let get_valid_orders = || {
            if let Some(orders) = get_orders(&manifest) {
                if !orders.is_empty() {
                    return Some(orders);
                }
            }
            None
        };

        if let Some(orders) = get_valid_orders() {
            (
                StatusCode::OK,
                orders
                    .iter()
                    .map(Order::to_string)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
        } else {
            (StatusCode::NO_CONTENT, String::new())
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
}
