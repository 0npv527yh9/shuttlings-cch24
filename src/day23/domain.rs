pub mod models {
    use strum::{Display, EnumString};

    #[derive(EnumString, Display)]
    #[strum(serialize_all = "snake_case")]
    pub enum Color {
        Red,
        Blue,
        Purple,
    }

    impl Color {
        pub fn into_next_color(self) -> Color {
            match self {
                Color::Red => Color::Blue,
                Color::Blue => Color::Purple,
                Color::Purple => Color::Red,
            }
        }
    }

    #[derive(EnumString, Display)]
    #[strum(serialize_all = "snake_case")]
    pub enum State {
        On,
        Off,
    }
}
