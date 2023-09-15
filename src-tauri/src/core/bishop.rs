use crate::core::piecemap::PieceMap;
use crate::utils::check::CheckMove;
use crate::utils::utils::*;
use std::cmp::max;
use std::sync::Mutex;
use tauri::State;

pub fn bishop_move_checker(
    map_mutex: State<'_, Mutex<PieceMap>>,
    start: &String,
    end: &String,
) -> bool {
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let start_index: i32 = PieceMap::tile_index(start.as_str()) as i32;
    let end_index: i32 = PieceMap::tile_index(end.as_str()) as i32;

    if map.map[end_index as usize] == 0000 || is_diff_color(&map, start_index, end_index) {
        if on_same_diagonal(start_index, end_index) {
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

pub fn possible_bishop_moves(map_mutex: State<'_, Mutex<PieceMap>>, tile: &String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
    let start_index: i32 = PieceMap::tile_index(tile.as_str()) as i32;
    let mut found_piece1: bool = false;
    let mut found_piece2: bool = false;
    let mut found_piece3: bool = false;
    let mut found_piece4: bool = false;

    let right: i32 = 8 - (file(PieceMap::tile_index(tile.as_str()) as i32));
    let left: i32 = 8 - (right + 1);
    let max: i32 = max(left, right);

    // top-left
    for l in 0..max {
        let left_up_index: i32 = start_index - ((l + 1) * 7);
        let left_down_index: i32 = start_index - ((l + 1) * 9);
        let right_up_index: i32 = start_index + ((l + 1) * 9);
        let right_down_index: i32 = start_index + ((l + 1) * 7);

        if on_same_diagonal(left_down_index, start_index) && left_down_index > 0 {
            bishop_calc(
                &map,
                start_index,
                left_down_index,
                &mut res,
                &mut found_piece1,
            )
        }

        if on_same_diagonal(left_up_index, start_index) && left_up_index > 0 {
            bishop_calc(
                &map,
                start_index,
                left_up_index,
                &mut res,
                &mut found_piece2,
            )
        }

        if on_same_diagonal(right_up_index, start_index) && right_up_index < 64 {
            bishop_calc(
                &map,
                start_index,
                right_up_index,
                &mut res,
                &mut found_piece3,
            )
        }

        if on_same_diagonal(right_down_index, start_index) && right_down_index < 64 {
            bishop_calc(
                &map,
                start_index,
                right_down_index,
                &mut res,
                &mut found_piece4,
            )
        }
    }

    return res;
}

pub fn on_same_diagonal(index1: i32, index2: i32) -> bool {
    (file(index1) - file(index2)).abs() == (rank(index1) - rank(index2)).abs()
}

fn bishop_calc(
    map: &std::sync::MutexGuard<'_, PieceMap>,
    i1: i32,
    i2: i32,
    res: &mut Vec<String>,
    found_piece: &mut bool,
) {
    if !is_diff_color(&map, i1, i2) {
        *found_piece = true;
        return;
    }
    if is_diff_color(&map, i1, i2) && !*found_piece && map.map[(i2) as usize] != 0000 {
        let n: String = PieceMap::tile_from_index(i2);
        res.push(n);
        *found_piece = true;
    }
    if map.map[(i2) as usize] == 0000 && !*found_piece {
        let n: String = PieceMap::tile_from_index(i2);
        res.push(n);
    }
}
