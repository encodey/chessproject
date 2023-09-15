use crate::core::piecemap::PieceMap;
use crate::utils::utils::*;
use std::sync::Mutex;
use tauri::State;

pub fn king_move_checker(
    map_mutex: State<'_, Mutex<PieceMap>>,
    start: &String,
    end: &String,
) -> bool {
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let start_index: i32 = PieceMap::tile_index(start.as_str()) as i32;
    let end_index: i32 = PieceMap::tile_index(end.as_str()) as i32;

    let attacked: bool = is_pawn_attacking(&map, &map.get_tile(&start).to_string(), end);
    if is_diff_color(&map, start_index, end_index)
        && !attacked
        && !map.attacked.contains(&end_index)
    {
        // need to check if the move walks into a check.
        return true;
    }

    return false;
}

pub fn possible_king_moves(tile: &String) -> Vec<String> {
    let start_index: i32 = PieceMap::tile_index(tile.as_str()) as i32;
    let mut valid_indexes: Vec<String> = Vec::new();

    // left
    if start_index > 7 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 8));
        if rank(start_index) != 8 {
            valid_indexes.push(PieceMap::tile_from_index(start_index - 7));
        }
        if rank(start_index) != 1 {
            valid_indexes.push(PieceMap::tile_from_index(start_index - 9));
        }
    }
    // right
    if start_index < 56 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 8));
        if rank(start_index) != 8 {
            valid_indexes.push(PieceMap::tile_from_index(start_index + 9));
        }
        if rank(start_index) != 1 {
            valid_indexes.push(PieceMap::tile_from_index(start_index + 7));
        }
    }
    // up
    if rank(start_index) != 8 {
        valid_indexes.push(PieceMap::tile_from_index(start_index + 1))
    }
    // down
    if rank(start_index) != 1 {
        valid_indexes.push(PieceMap::tile_from_index(start_index - 1))
    }

    return valid_indexes;
}

fn is_pawn_attacking(
    map: &std::sync::MutexGuard<'_, PieceMap>,
    piece: &String,
    target: &String,
) -> bool {
    //let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();

    if piece.chars().nth(0).unwrap() == 'W' {
        // In this case, the king is white
        let index: i32 = PieceMap::tile_index(&target) as i32;
        if rank(index) < 8 && file(index) > 1 {
            let first_index: i32 = index + 9;
            if map.map[first_index as usize] == 7 {
                return true;
            }
        }
        if rank(index) < 8 && file(index) < 8 {
            let second_index: i32 = index - 7;
            if map.map[second_index as usize] == 7 {
                return true;
            }
        }
    }

    return false;
}
