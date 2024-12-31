pub mod model {
    use serde::Serialize;
    use sqlx::{
        prelude::FromRow,
        types::chrono::{DateTime, Utc},
    };
    use uuid::Uuid;

    #[derive(Serialize, FromRow)]
    pub struct Quote {
        pub id: Uuid,
        pub author: String,
        pub quote: String,
        pub created_at: DateTime<Utc>,
        pub version: i32,
    }

    #[derive(Serialize, FromRow)]
    pub struct QuoteList {
        pub quotes: Vec<Quote>,
        pub page: i32,
        pub next_token: Option<String>,
    }
}

pub mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UpdateQuote {
        pub author: String,
        pub quote: String,
    }

    #[derive(Deserialize)]
    pub struct InsertQuote {
        pub author: String,
        pub quote: String,
    }

    #[derive(Deserialize)]
    pub struct Token {
        pub token: Option<String>,
    }
}
