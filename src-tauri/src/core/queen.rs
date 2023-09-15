use crate::core::bishop::*;
use crate::core::piecemap::PieceMap;
use crate::core::rook::*;
use std::sync::Mutex;
use tauri::State;

pub fn possible_queen_moves(map_mutex: State<'_, Mutex<PieceMap>>, tile: &String) -> Vec<String> {
    let mut moves: Vec<String> = possible_rook_moves(map_mutex.clone(), tile);
    let mut diagonal_moves: Vec<String> = possible_bishop_moves(map_mutex.clone(), tile);

    moves.append(&mut diagonal_moves);
    return moves;
}

pub fn queen_move_checker(
    map_mutex: State<'_, Mutex<PieceMap>>,
    start: &String,
    end: &String,
) -> bool {
    rook_move_checker(map_mutex.clone(), start, end)
        || bishop_move_checker(map_mutex.clone(), start, end)
}
