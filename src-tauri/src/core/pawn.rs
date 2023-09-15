use crate::core::piecemap::PieceMap;
use crate::utils::check::CheckMove;
use std::sync::Mutex;
use tauri::State;

pub fn pawn_move_checker(
    map_mutex: State<'_, Mutex<PieceMap>>,
    start: &String,
    end: &String,
) -> bool {
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let mut _on_start: bool = false;
    let piece: &str = map.get_tile(start.as_str());

    if piece.starts_with('W') && start.contains("2") {
        _on_start = true;
    } else if piece.starts_with('B') && start.contains("7") {
        _on_start = true;
    }

    let end_index: i32 = PieceMap::tile_index(end.as_str()) as i32;
    let start_index: i32 = PieceMap::tile_index(start.as_str()) as i32;

    // Check for captures
    if piece.starts_with('W') {
        if (end_index - start_index == 9 || end_index - start_index == -7)
            && map.get_tile(end.as_str()).starts_with('B')
        {
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
        // standard move
        if (_on_start && end_index - start_index == 2)
            && PieceMap::convert_to_piecemap(map.get_tile(end.as_str()).to_string()) == 0000
            && map.map[(end_index - 1) as usize] == 0000
        {
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
        if end_index - start_index == 1
            && PieceMap::convert_to_piecemap(map.get_tile(end.as_str()).to_string()) == 0000
        {
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
    }
    if piece.starts_with('B') {
        if (end_index - start_index == -9 || end_index - start_index == 7)
            && map.get_tile(end.as_str()).starts_with('W')
        {
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
        // standard move
        if (_on_start && end_index - start_index == -2)
            && PieceMap::convert_to_piecemap(map.get_tile(end.as_str()).to_string()) == 0000
            && map.map[(end_index + 1) as usize] == 0000
        {
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
        if end_index - start_index == -1
            && PieceMap::convert_to_piecemap(map.get_tile(end.as_str()).to_string()) == 0000
        {
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
    }

    // Check for standard moves
    false
}

pub fn possible_pawn_moves(tile: &String) -> Vec<String> {
    let start_index: i32 = PieceMap::tile_index(tile.as_str()) as i32;
    let mut valid_indexes: Vec<String> = Vec::new();
    if start_index > 0 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 1));
    }
    if start_index > 1 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 2));
    }
    if start_index > 6 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 7));
    }
    if start_index > 8 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 9));
    }
    if start_index < 63 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 1));
    }
    if start_index < 62 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 2));
    }
    if start_index < 57 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 7));
    }
    if start_index < 55 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 9));
    }

    return valid_indexes;
}
