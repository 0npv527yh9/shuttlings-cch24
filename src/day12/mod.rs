mod board;
mod entity;

use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
};
use board::{Board, BoardRng};
use entity::{PlaceResponse, Team};
use rand::rngs::StdRng;
use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

pub async fn board(state: extract::State<Arc<Mutex<State>>>) -> impl IntoResponse {
    state.lock().unwrap().board.to_string()
}

pub async fn reset(state: extract::State<Arc<Mutex<State>>>) -> impl IntoResponse {
    let mut guard = state.lock().unwrap();
    let State { board, rng } = guard.deref_mut();
    board.reset();
    *rng = StdRng::new();
    board.to_string()
}

pub async fn place(
    state: extract::State<Arc<Mutex<State>>>,
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

pub async fn random_board(state: extract::State<Arc<Mutex<State>>>) -> impl IntoResponse {
    let mut guard = state.lock().unwrap();
    let State { board, rng } = guard.deref_mut();
    board.make_random(rng);
    board.to_string()
}

pub fn create_state() -> State {
    State {
        board: Board::new(),
        rng: StdRng::new(),
    }
}

pub struct State {
    board: Board,
    rng: StdRng,
}
