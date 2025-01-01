use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, PartialEq, Copy)]
pub enum Tile {
    Empty,
    Piece(Team),
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "⬛"),
            Tile::Wall => write!(f, "⬜"),
            Tile::Piece(team) => write!(f, "{team}"),
        }
    }
}

impl From<Team> for Tile {
    fn from(team: Team) -> Self {
        Tile::Piece(team)
    }
}

#[derive(Clone, Deserialize, PartialEq, strum_macros::Display, Copy)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Team {
    #[strum(serialize = "cookie", to_string = "🍪")]
    Cookie,
    #[strum(serialize = "milk", to_string = "🥛")]
    Milk,
}

#[derive(Clone, Copy)]
pub enum BoardState {
    Playing,
    Finished(Option<Team>),
}

pub enum PlaceError {
    AlreadyFinished,
    InvalidColumn,
    FulledColumn,
}
