use super::entity::{BoardState, PlaceResponse, Team, Tile};
use itertools::Itertools;
use std::{fmt::Display, iter};

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    state: BoardState,
}

impl Board {
    const H: usize = 4;
    const W: usize = 4;

    pub fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; Self::W]; Self::H],
            state: BoardState::Playing,
        }
    }

    pub fn reset(&mut self) {
        self.tiles.iter_mut().for_each(|row| row.fill(Tile::Empty));
        self.state = BoardState::Playing;
    }

    pub fn place(&mut self, team: Team, column: usize) -> PlaceResponse {
        let is_valid_column = (1..=Board::W).contains(&column);

        if !is_valid_column {
            PlaceResponse::InvalidColumn
        } else {
            let col = column - 1;

            let empty_row = (0..Board::H)
                .rev()
                .find(|&i| self.tiles[i][col] == Tile::Empty);

            match empty_row {
                None => PlaceResponse::FulledColumn,
                Some(row) => match self.state {
                    BoardState::Finished(_) => PlaceResponse::AlreadyFinished,
                    BoardState::Playing => {
                        let tile = Tile::from(team);
                        self.tiles[row][col] = tile;
                        self.state = self.state_after_place(row, col, team, tile);
                        PlaceResponse::Ok
                    }
                },
            }
        }
    }

    fn state_after_place(&self, row: usize, col: usize, team: Team, tile: Tile) -> BoardState {
        let is_col_covered = (0..Self::H).all(|i| self.tiles[i][col] == tile);
        let is_row_covered = (0..Self::W).all(|j| self.tiles[row][j] == tile);
        let is_diag1_covered = row == col && (0..Self::H).all(|k| self.tiles[k][k] == tile);
        let is_diag2_covered = row + col == Self::H - 1
            && (0..Self::H).all(|k| self.tiles[k][Self::W - 1 - k] == tile);

        if is_col_covered || is_row_covered || is_diag1_covered || is_diag2_covered {
            BoardState::Finished(Some(team))
        } else if self
            .tiles
            .iter()
            .flat_map(|row| row.iter())
            .all(|&tile| tile != Tile::Empty)
        {
            BoardState::Finished(None)
        } else {
            self.state
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = self
            .tiles
            .iter()
            .map(|row| {
                iter::once(&Tile::Wall)
                    .chain(row.iter())
                    .chain(iter::once(&Tile::Wall))
                    .join("")
            })
            .chain(iter::once(
                iter::repeat(Tile::Wall).take(Self::W + 2).join(""),
            ))
            .collect::<Vec<_>>();

        if let BoardState::Finished(winner) = self.state {
            lines.push(winner_message(winner));
        }

        writeln!(f, "{}", lines.join("\n"))
    }
}

fn winner_message(winner: Option<Team>) -> String {
    match winner {
        Some(team) => format!("{team} wins!"),
        None => "No winner.".to_string(),
    }
}
