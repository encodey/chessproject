use crate::{
    core::{bishop::on_same_diagonal, piecemap::PieceMap},
    utils::utils::{file, rank},
};
use std::sync::Mutex;
use tauri::State;

pub struct CheckMove {
    attacker: String,
    target: String,
    active: bool,
}

impl CheckMove {
    pub fn new() -> Self {
        CheckMove {
            attacker: String::new(),
            target: String::new(),
            active: false,
        }
    }

    pub fn set_attacker(mut self, tile: String) -> Self {
        self.attacker = tile;
        self
    }

    pub fn set_target(mut self, tile: String) -> Self {
        self.target = tile;
        self
    }

    pub fn get_attacker(&self) -> String {
        self.attacker.to_string()
    }

    pub fn get_target(&self) -> String {
        self.target.to_string()
    }

    pub fn set_active(&mut self, state: bool) -> &mut Self {
        self.active = state;
        self
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn set_check(&mut self, target: &String, attacker: &String) {
        println!("Setting a: {} | t: {}", attacker, target);
        self.attacker = attacker.to_string();
        self.target = target.to_string();
        self.active = true
    }

    pub fn remove_check(&mut self) {
        self.active = false;
        self.attacker = String::new();
        self.target = String::new();
    }

    pub fn is_check(map_mutex: State<'_, Mutex<PieceMap>>, target: &String) -> bool {
        let map: std::sync::MutexGuard<'_, PieceMap> = map_mutex.lock().unwrap();
        if map.get_tile(&target).contains("King") {
            return true;
        }
        return false;
    }

    pub fn check_ray(target: &String, attacker: &String) -> Vec<String> {
        let t_index: i32 = PieceMap::tile_index(&target) as i32;
        let a_index: i32 = PieceMap::tile_index(&attacker) as i32;
        let mut ray: Vec<String> = Vec::new();

        if on_same_diagonal(t_index, a_index) {
            // In this case, return all the tiles between. If diff > 0 attacker is left, else it is right of king.
            let diff: i32 = file(t_index) - file(a_index);
            // Push tile of attacker (for taking)
            ray.push(PieceMap::tile_from_index(a_index));
            // loop over diff
            for i in 1..diff.abs() {
                if diff > 0 {
                    // att left
                    if rank(a_index) > rank(t_index) {
                        // att below
                        let tile: String = PieceMap::tile_from_index(a_index + (7 * i));
                        ray.push(tile)
                    } else {
                        // att above
                        let tile: String = PieceMap::tile_from_index(a_index + (9 * i));
                        ray.push(tile)
                    }
                } else {
                    // att right
                    if rank(a_index) > rank(t_index) {
                        // att below
                        let tile: String = PieceMap::tile_from_index(a_index - (9 * i));
                        ray.push(tile)
                    } else {
                        // att above
                        let tile: String = PieceMap::tile_from_index(a_index - (7 * i));
                        ray.push(tile)
                    }
                }
            }
        }

        if file(t_index) == file(a_index) {
            // vert attack
            let diff: i32 = rank(t_index) - rank(a_index);
            ray.push(PieceMap::tile_from_index(a_index));

            for i in 1..diff.abs() {
                if diff > 0 {
                    // att below
                    let tile: String = PieceMap::tile_from_index(a_index + i);
                    ray.push(tile)
                } else {
                    // att above
                    let tile: String = PieceMap::tile_from_index(a_index - i);
                    ray.push(tile)
                }
            }
        }

        if rank(t_index) == rank(a_index) {
            // horizontal attack
            let diff: i32 = file(t_index) - file(a_index);
            ray.push(PieceMap::tile_from_index(a_index));

            for i in 1..diff.abs() {
                if diff > 0 {
                    // att left
                    let tile: String = PieceMap::tile_from_index(a_index + (8 * i));
                    ray.push(tile)
                } else {
                    // att right
                    let tile: String = PieceMap::tile_from_index(a_index - (8 * i));
                    ray.push(tile)
                }
            }
        }

        return ray;
    }
}
