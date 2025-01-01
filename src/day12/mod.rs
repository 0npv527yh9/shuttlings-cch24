mod board;
mod entity;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use board::{Board, BoardRng};
use entity::{PlaceResponse, Team};
use rand::{rngs::StdRng, SeedableRng};
use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

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
) -> impl IntoResponse {
    let board = &mut state.lock().unwrap().board;

    match board.place(team, column) {
        PlaceResponse::InvalidColumn => (StatusCode::BAD_REQUEST, String::new()),
        PlaceResponse::AlreadyFinished | PlaceResponse::FulledColumn => {
            (StatusCode::SERVICE_UNAVAILABLE, board.to_string())
        }
        PlaceResponse::Ok => (StatusCode::OK, board.to_string()),
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

pub struct AppState {
    board: Board,
    rng: StdRng,
}
