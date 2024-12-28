use axum::{extract::State, response::IntoResponse};
use itertools::Itertools;
use std::{
    fmt::Display,
    iter,
    sync::{Arc, Mutex},
};

pub async fn board(board: State<Arc<Mutex<Board>>>) -> impl IntoResponse {
    board.lock().unwrap().to_string()
}

pub async fn reset(board: State<Arc<Mutex<Board>>>) -> impl IntoResponse {
    let mut board = board.lock().unwrap();
    board.reset();
    board.to_string()
}

pub fn create_board() -> Board {
    Board::new()
}

pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    const H: usize = 4;
    const W: usize = 4;

    fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; Self::W]; Self::H],
        }
    }

    fn reset(&mut self) {
        self.tiles.iter_mut().for_each(|row| row.fill(Tile::Empty));
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .iter()
            .map(|row| {
                iter::once(WALL)
                    .chain(row.iter().map(Tile::to_char))
                    .chain(iter::once(WALL))
                    .join("")
            })
            .chain(iter::once(iter::repeat(WALL).take(Self::W + 2).join("")))
            .join("\n");

        writeln!(f, "{s}")
    }
}

#[derive(Clone)]
enum Tile {
    Empty,
    Cookie,
    Milk,
}

const WALL: char = '⬜';

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '⬛',
            Tile::Cookie => '🍪',
            Tile::Milk => '🥛',
        }
    }
}
