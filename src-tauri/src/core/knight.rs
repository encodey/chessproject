use crate::core::piecemap::PieceMap;
use crate::utils::check::CheckMove;
use crate::utils::utils::*;
use std::sync::Mutex;
use tauri::State;

pub fn knight_move_checker(
    map_mutex: State<'_, Mutex<PieceMap>>,
    start: &String,
    end: &String,
) -> bool {
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let start_index: i32 = PieceMap::tile_index(start.as_str()) as i32;
    let end_index: i32 = PieceMap::tile_index(end.as_str()) as i32;

    if is_diff_color(&map, start_index, end_index) {
        if !map.check.get_active() {
            return true;
        } else {
            let blocks: Vec<String> =
                CheckMove::check_ray(&map.check.get_target(), &map.check.get_attacker());

            if blocks.contains(end) {
                return true;
            }
        }
    }

    return false;
}

pub fn possible_knight_moves(tile: &String) -> Vec<String> {
    let start_index: i32 = PieceMap::tile_index(tile.as_str()) as i32;
    let mut valid_indexes: Vec<String> = Vec::new();

    if rank(start_index) < 7 && file(start_index) > 1 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 6))
    }
    if rank(start_index) < 7 && file(start_index) < 8 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 10))
    }
    if rank(start_index) > 2 && file(start_index) > 1 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 10))
    }
    if rank(start_index) > 2 && file(start_index) < 8 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 6))
    }

    if file(start_index) < 7 && rank(start_index) < 8 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 17))
    }
    if file(start_index) < 7 && rank(start_index) > 1 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 15))
    }
    if file(start_index) > 2 && rank(start_index) < 8 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 15))
    }
    if file(start_index) > 2 && rank(start_index) > 2 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 17))
    }

    return valid_indexes;
}
