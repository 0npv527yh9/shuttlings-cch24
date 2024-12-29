use super::entity::{BoardState, PlaceResponse, Team, Tile};
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{fmt::Display, iter};

pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    const H: usize = 4;
    const W: usize = 4;

    pub fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; Self::W]; Self::H],
        }
    }

    pub fn reset(&mut self) {
        self.tiles.iter_mut().for_each(|row| row.fill(Tile::Empty));
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
                Some(row) => match self.state() {
                    BoardState::Finished(_) => PlaceResponse::AlreadyFinished,
                    BoardState::Playing => {
                        let tile = Tile::from(team);
                        self.tiles[row][col] = tile;
                        PlaceResponse::Ok
                    }
                },
            }
        }
    }

    pub fn make_random(&mut self, rng: &mut StdRng) {
        for i in 0..Self::H {
            for j in 0..Self::W {
                let team = rng.gen_team();
                self.tiles[i][j] = Tile::from(team);
            }
        }
    }

    fn state(&self) -> BoardState {
        for team in [Team::Milk, Team::Cookie] {
            let tile = Tile::from(team);

            for i in 0..Self::H {
                if (0..Self::W).all(|j| self.tiles[i][j] == tile) {
                    return BoardState::Finished(Some(team));
                }
            }
            for j in 0..Self::W {
                if (0..Self::H).all(|i| self.tiles[i][j] == tile) {
                    return BoardState::Finished(Some(team));
                }
            }
            if (0..Self::H).all(|k| self.tiles[k][k] == tile) {
                return BoardState::Finished(Some(team));
            }
            if (0..Self::H).all(|k| self.tiles[k][Self::W - 1 - k] == tile) {
                return BoardState::Finished(Some(team));
            }
        }

        if self
            .tiles
            .iter()
            .flat_map(|row| row.iter())
            .all(|&tile| tile != Tile::Empty)
        {
            BoardState::Finished(None)
        } else {
            BoardState::Playing
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

        if let BoardState::Finished(winner) = self.state() {
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

pub trait BoardRng {
    fn new() -> Self;
    fn gen_team(&mut self) -> Team;
}

impl BoardRng for StdRng {
    fn new() -> Self {
        StdRng::seed_from_u64(2024)
    }

    fn gen_team(&mut self) -> Team {
        if self.gen::<bool>() {
            Team::Cookie
        } else {
            Team::Milk
        }
    }
}
