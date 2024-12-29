mod board;
mod entity;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use board::Board;
use entity::{PlaceResponse, Team};
use std::sync::{Arc, Mutex};

pub async fn board(board: State<Arc<Mutex<Board>>>) -> impl IntoResponse {
    board.lock().unwrap().to_string()
}

pub async fn reset(board: State<Arc<Mutex<Board>>>) -> impl IntoResponse {
    let mut board = board.lock().unwrap();
    board.reset();
    board.to_string()
}

pub async fn place(
    board: State<Arc<Mutex<Board>>>,
    Path((team, column)): Path<(Team, usize)>,
) -> impl IntoResponse {
    let mut board = board.lock().unwrap();

    match board.place(team, column) {
        PlaceResponse::InvalidColumn => (StatusCode::BAD_REQUEST, String::new()),
        PlaceResponse::AlreadyFinished | PlaceResponse::FulledColumn => {
            (StatusCode::SERVICE_UNAVAILABLE, board.to_string())
        }
        PlaceResponse::Ok => (StatusCode::OK, board.to_string()),
    }
}

pub fn create_board() -> Board {
    Board::new()
}
