// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod core {
    pub mod bishop;
    pub mod king;
    pub mod knight;
    pub mod pawn;
    pub mod piecemap;
    pub mod queen;
    pub mod rook;
}
mod utils {
    pub mod check;
    pub mod utils;
}

use crate::core::bishop::{bishop_move_checker, possible_bishop_moves};
use crate::core::king::{king_move_checker, possible_king_moves};
use crate::core::knight::{knight_move_checker, possible_knight_moves};
use crate::core::pawn::{pawn_move_checker, possible_pawn_moves};
use crate::core::piecemap::PieceMap;
use crate::core::queen::{possible_queen_moves, queen_move_checker};
use crate::core::rook::{possible_rook_moves, rook_move_checker};
use std::sync::Mutex;
use tauri::State;
use utils::check::CheckMove;

fn main() {
    // Create our default PieceMap for the whole game.
    let piece_map: PieceMap = PieceMap::new();

    tauri::Builder::default()
        .manage(Mutex::new(piece_map))
        .invoke_handler(tauri::generate_handler![
            set_tile,
            manage_attacker,
            get_possible_moves,
            legal_move_checker,
            reset,
            is_check,
            set_check,
            rem_check,
            get_current,
            set_current
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_tile(map: State<'_, Mutex<PieceMap>>, tile: &str, piece: String) {
    map.lock().unwrap().set_tile(tile, piece.clone());

    // Just moved a piece to block a check
    if map.lock().unwrap().check.get_active() && !piece.contains("King") {
        // let att = &map.lock().unwrap().attacked;
        // for tile in att {
        //     map.lock().unwrap().attacked.remove(*tile as usize);
        // }
    }
}

#[tauri::command]
fn legal_move_checker(map: State<'_, Mutex<PieceMap>>, start: String, end: String) -> bool {
    let piece: &str = map.lock().unwrap().get_tile(&start);
    match piece {
        "WPawn" => pawn_move_checker(map, &start, &end),
        "BPawn" => pawn_move_checker(map, &start, &end),
        "WRook" => rook_move_checker(map, &start, &end),
        "BRook" => rook_move_checker(map, &start, &end),
        "WBishop" => bishop_move_checker(map, &start, &end),
        "BBishop" => bishop_move_checker(map, &start, &end),
        "WQueen" => queen_move_checker(map, &start, &end),
        "BQueen" => queen_move_checker(map, &start, &end),
        "WKing" => king_move_checker(map, &start, &end),
        "BKing" => king_move_checker(map, &start, &end),
        "WKnight" => knight_move_checker(map, &start, &end),
        "BKnight" => knight_move_checker(map, &start, &end),
        _ => false,
    }
}

#[tauri::command]
fn get_possible_moves(map: State<'_, Mutex<PieceMap>>, tile: String, piece: String) -> Vec<String> {
    match piece.as_str() {
        "WPawn" => possible_pawn_moves(&tile),
        "BPawn" => possible_pawn_moves(&tile),
        "WRook" => possible_rook_moves(map, &tile),
        "BRook" => possible_rook_moves(map, &tile),
        "WBishop" => possible_bishop_moves(map, &tile),
        "BBishop" => possible_bishop_moves(map, &tile),
        "WQueen" => possible_queen_moves(map, &tile),
        "BQueen" => possible_queen_moves(map, &tile),
        "WKing" => possible_king_moves(&tile),
        "BKing" => possible_king_moves(&tile),
        "WKnight" => possible_knight_moves(&tile),
        "BKnight" => possible_knight_moves(&tile),
        _ => Vec::<String>::new(),
    }
}

#[tauri::command]
fn reset(map: State<'_, Mutex<PieceMap>>) {
    map.lock().unwrap().reset()
}

#[tauri::command]
fn is_check(map_mutex: State<'_, Mutex<PieceMap>>, target: String) -> bool {
    CheckMove::is_check(map_mutex, &target)
}

#[tauri::command]
fn set_check(map_mutex: State<'_, Mutex<PieceMap>>, attacker: String, target: String) {
    map_mutex
        .lock()
        .unwrap()
        .check
        .set_check(&target, &attacker);
    let r = CheckMove::check_ray(&target.clone(), &attacker.clone());
    for i in r {
        println!("{}", i);
    }
    println!("current attacked: {:?}", map_mutex.lock().unwrap().attacked);
}

#[tauri::command]
fn rem_check(map_mutex: State<'_, Mutex<PieceMap>>) {
    map_mutex.lock().unwrap().check.remove_check()
}

#[tauri::command]
fn get_current(map_mutex: State<'_, Mutex<PieceMap>>) -> bool {
    map_mutex.lock().unwrap().current
}

#[tauri::command]
fn set_current(map_mutex: State<'_, Mutex<PieceMap>>, new: bool) {
    map_mutex.lock().unwrap().current = new
}

#[tauri::command]
fn manage_attacker(map_mutex: State<'_, Mutex<PieceMap>>, tile: String) {
    let index: i32 = PieceMap::tile_index(&tile) as i32;
    map_mutex.lock().unwrap().manage_attacker(index)
}
