use crate::core::piecemap::PieceMap;
use crate::utils::check::CheckMove;
use crate::utils::utils::*;
use std::sync::Mutex;
use tauri::State;

pub fn rook_move_checker(
    map_mutex: State<'_, Mutex<PieceMap>>,
    start: &String,
    end: &String,
) -> bool {
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let start_index: i32 = PieceMap::tile_index(start.as_str()) as i32;
    let end_index: i32 = PieceMap::tile_index(end.as_str()) as i32;

    if file(start_index) == file(end_index) || rank(start_index) == rank(end_index) {
        if PieceMap::convert_from_piecemap(map.map[start_index as usize])
            .chars()
            .nth(0)
            != PieceMap::convert_from_piecemap(map.map[end_index as usize])
                .chars()
                .nth(0)
            || map.map[end_index as usize] == 0000
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

    return false;
}

pub fn possible_rook_moves(map_mutex: State<'_, Mutex<PieceMap>>, tile: &String) -> Vec<String> {
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let start_index: i32 = PieceMap::tile_index(tile.as_str()) as i32;
    let file: i32 = file(start_index);
    let rank: i32 = rank(start_index);
    let mut valid_indexes: Vec<String> = Vec::new();
    let mut found_piece: bool = false;

    // check going up
    for i in start_index..start_index + (8 - rank) + 1 {
        if i != start_index {
            if map.map[i as usize] != 0000 && !found_piece {
                if PieceMap::convert_from_piecemap(map.map[i as usize])
                    .chars()
                    .nth(0)
                    != PieceMap::convert_from_piecemap(map.map[start_index as usize])
                        .chars()
                        .nth(0)
                {
                    let n: String = PieceMap::tile_from_index(i);
                    valid_indexes.push(n);
                }
                found_piece = true;
            } else {
                if !found_piece {
                    let n: String = PieceMap::tile_from_index(i);
                    valid_indexes.push(n);
                }
            }
        }
    }
    found_piece = false;

    // check going down
    let start: i32;
    if start_index - rank <= 0 {
        start = 0;
    } else {
        start = start_index - rank;
    }
    for i in (start..start_index).rev() {
        if i != start_index {
            if map.map[i as usize] != 0000 && !found_piece {
                if PieceMap::convert_from_piecemap(map.map[i as usize])
                    .chars()
                    .nth(0)
                    != PieceMap::convert_from_piecemap(map.map[start_index as usize])
                        .chars()
                        .nth(0)
                {
                    let n: String = PieceMap::tile_from_index(i);
                    valid_indexes.push(n);
                }
                found_piece = true;
            } else if !found_piece {
                let n: String = PieceMap::tile_from_index(i);
                valid_indexes.push(n);
            }
        }
    }
    found_piece = false;

    //check right
    for i in (start_index..start_index + 8 * (8 - file) + 1).step_by(8) {
        if i != start_index {
            if map.map[i as usize] != 0000 && !found_piece {
                if PieceMap::convert_from_piecemap(map.map[i as usize])
                    .chars()
                    .nth(0)
                    != PieceMap::convert_from_piecemap(map.map[start_index as usize])
                        .chars()
                        .nth(0)
                {
                    let n: String = PieceMap::tile_from_index(i);
                    valid_indexes.push(n);
                }
                found_piece = true;
            } else if !found_piece {
                let n: String = PieceMap::tile_from_index(i);
                valid_indexes.push(n);
            }
        }
    }
    found_piece = false;

    // check left
    for i in (0..start_index).rev().step_by(8) {
        if i + 1 != start_index {
            if map.map[(i + 1) as usize] != 0000 && !found_piece {
                if PieceMap::convert_from_piecemap(map.map[(i + 1) as usize])
                    .chars()
                    .nth(0)
                    != PieceMap::convert_from_piecemap(map.map[start_index as usize])
                        .chars()
                        .nth(0)
                {
                    let n: String = PieceMap::tile_from_index(i + 1);
                    valid_indexes.push(n);
                }
                found_piece = true;
            } else if !found_piece {
                let n: String = PieceMap::tile_from_index(i + 1);
                valid_indexes.push(n);
            }
        }
    }

    return valid_indexes;
}
