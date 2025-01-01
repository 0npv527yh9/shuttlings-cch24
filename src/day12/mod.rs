mod board;
mod entity;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use board::{Board, BoardRng};
use entity::{PlaceError, Team};
use rand::{rngs::StdRng, SeedableRng};
use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

pub struct AppState {
    board: Board,
    rng: StdRng,
}

pub async fn board(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    state.lock().unwrap().board.to_string()
}

pub async fn reset(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut guard = state.lock().unwrap();
    let AppState { board, rng } = guard.deref_mut();
    board.reset();
    *rng = StdRng::new();
    board.to_string()
}

pub async fn place(
    state: State<Arc<Mutex<AppState>>>,
    Path((team, column)): Path<(Team, usize)>,
) -> Result<String, (StatusCode, String)> {
    let board = &mut state.lock().unwrap().board;

    match board.place(team, column) {
        Ok(_) => Ok(board.to_string()),
        Err(error) => Err((error.into(), board.to_string())),
    }
}

impl From<PlaceError> for StatusCode {
    fn from(value: PlaceError) -> Self {
        match value {
            PlaceError::InvalidColumn => StatusCode::BAD_REQUEST,
            PlaceError::AlreadyFinished | PlaceError::FulledColumn => {
                StatusCode::SERVICE_UNAVAILABLE
            }
        }
    }
}

pub async fn random_board(state: State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut guard = state.lock().unwrap();
    let AppState { board, rng } = guard.deref_mut();
    board.make_random(rng);
    board.to_string()
}

pub fn create_state() -> AppState {
    AppState {
        board: Board::new(),
        rng: StdRng::seed_from_u64(2024),
    }
}
