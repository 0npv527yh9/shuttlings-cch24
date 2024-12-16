pub mod task1 {

    use axum::http::StatusCode;
    use serde::Deserialize;
    use toml::{Table, Value};

    #[derive(Deserialize)]
    struct Manifest {
        package: Package,
    }

    #[derive(Deserialize)]
    struct Package {
        name: String,
        authors: Vec<String>,
        keywords: Vec<String>,
        metadata: Metadata,
    }

    #[derive(Deserialize)]
    struct Metadata {
        orders: Option<Vec<Option<Order>>>,
    }

    #[derive(Deserialize, Debug)]
    struct Order {
        item: String,
        quantity: u32,
    }

    pub async fn manifest(body: String) -> (StatusCode, String) {
        println!("{:?}", body);
        let manifest = body.parse::<Table>().unwrap();

        let package = manifest.get("package").unwrap();
        let metadata = match package.get("metadata") {
            Some(metadata) => metadata,
            None => return (StatusCode::NO_CONTENT, String::new()),
        };

        let orders = metadata.get("orders");
        let mut res = vec![];
        if let Some(Value::Array(orders)) = orders {
            for order in orders {
                if let Value::Table(order) = order {
                    if let Some(Value::String(item)) = order.get("item") {
                        if let Some(Value::Integer(quantity)) = order.get("quantity") {
                            res.push(format!("{}: {}", item, quantity));
                        }
                    }
                }
            }
            if !res.is_empty() {
                return (StatusCode::OK, res.join("\n"));
            }
        }

        (StatusCode::NO_CONTENT, String::new())
    }
}
