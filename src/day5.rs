pub mod task1 {

    use std::fmt::Display;

    use axum::http::StatusCode;
    use toml::{Table, Value};

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
        let manifest = body.parse::<Table>().unwrap();
        println!("{manifest:?}");

        let parse = || -> Option<Vec<Order>> {
            let orders = manifest
                .get("package")?
                .get("metadata")?
                .get("orders")?
                .as_array()?
                .iter()
                .filter_map(|order| {
                    if let Value::Table(order) = order {
                        if let (Some(Value::String(item)), Some(Value::Integer(quantity))) =
                            (order.get("item"), order.get("quantity"))
                        {
                            return Some(Order {
                                item: item.to_owned(),
                                quantity: *quantity as u32,
                            });
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();
            Some(orders)
        };

        if let Some(orders) = parse() {
            if !orders.is_empty() {
                return (
                    StatusCode::OK,
                    orders
                        .iter()
                        .map(Order::to_string)
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
            }
        }
        (StatusCode::NO_CONTENT, String::new())
    }
}
