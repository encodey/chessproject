use crate::utils::check::CheckMove;
use crate::utils::utils::*;


/// A PieceMap represents all the pieces on the board in one array consisting of 64 u8-type values.
///
/// Each u8 value represents a piece mapped to a binary number.
///
/// For each pieces mapped representation see the functions convert_to_piecemap() and convert_from_piecemap()
///
/// The order of tiles in the map is a1->a8, then b1->b8 ect.
/// The `attacked` vector represents indexes (in i32 type) from the map where the tile is under attack from a piece, and thus is an illegal king move.
///
pub struct PieceMap {
    pub map: [u8; 64],
    pub check: CheckMove,
    pub current: bool,
    pub attacked: Vec<i32>,
}

impl PieceMap {
    /// Takes a string input of the name of a piece and converts it to its piecemap representation.
    ///
    /// # Arguments
    /// * `piece: String`   The string representation of the piece.
    ///
    /// # Example
    ///
    /// ```
    /// let x: String = "BPawn".into();
    /// let y: &str = convert_to_piecemap(x);
    /// println!("{:?}", y);
    /// ```
    /// Would print `0111` to the standard output.
    ///
    pub fn convert_to_piecemap(piece: String) -> u8 {
        return match piece.as_str() {
            "WPawn" => 0b0001,
            "WBishop" => 0b0010,
            "WKnight" => 0b0011,
            "WRook" => 0b0100,
            "WQueen" => 0b0101,
            "WKing" => 0b0110,
            "BPawn" => 0b0111,
            "BBishop" => 0b1000,
            "BKnight" => 0b1001,
            "BRook" => 0b1010,
            "BQueen" => 0b1011,
            "BKing" => 0b1100,
            _ => 0b0000,
        };
    }

    /// Takes a piecemap representation of a piece and converts it back to a string.
    ///
    /// # Arguments
    /// * `piece: String`   The string representation of the piece.
    ///
    /// # Example
    ///
    /// ```
    /// let x: String = "0011".into();
    /// let y: &str = convert_from_piecemap(x);
    /// println!("{:?}", y);
    /// ```
    /// Would print `WKnight` to the standard output.
    ///
    pub fn convert_from_piecemap(piece: u8) -> &'static str {
        return match piece {
            0b0001 => "WPawn",
            0b0010 => "WBishop",
            0b0011 => "WKnight",
            0b0100 => "WRook",
            0b0101 => "WQueen",
            0b0110 => "WKing",
            0b0111 => "BPawn",
            0b1000 => "BBishop",
            0b1001 => "BKnight",
            0b1010 => "BRook",
            0b1011 => "BQueen",
            0b1100 => "BKing",
            _ => "",
        };
    }

    /// Returns a new piecemap with all tiles empty.
    ///
    /// # Example
    /// ```
    /// let map: PieceMap = PieceMap::new();
    /// ```
    pub fn new() -> Self {
        PieceMap {
            map: [
                0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
                0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
                0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
                0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
                0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
                0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
                0b0000, 0b0000, 0b0000, 0b0000,
            ],
            check: CheckMove::new(),
            current: true,
            attacked: Vec::new(),
        }
    }

    /// Returns the piece currently on the tile. Arguments take the standard notation eg. "a3" "BRook".
    ///
    /// # Arguments
    /// * `tile: &str`  The tile to find the current piece on.
    ///
    /// # Example
    /// ```
    /// let map: PieceMap = PieceMap::new();
    /// let x: &str = map.get_piece_on_tile("a2")
    /// println!("{:?}", x);
    /// ```
    /// Would print "WPawn" on a starting position board.
    ///
    pub fn get_tile(&self, tile: &str) -> &'static str {
        let tile_index: usize = PieceMap::tile_index(tile);
        let mapped_piece: u8 = self.map[tile_index];
        let piece: &str = PieceMap::convert_from_piecemap(mapped_piece);
        piece
    }

    /// Sets the piece currently on the tile. Arguments take the standard notation eg. "a3" "BRook".
    ///
    /// # Arguments
    /// * `tile: &str`  The tile to place the current piece on.
    /// * `piece: String`   The piece to place on the tile.
    ///
    /// # Example
    /// ```
    /// let map: PieceMap = PieceMap::new();
    /// map.set_tile("a3", "WPawn".into())
    /// ```
    /// Would move a white pawn to the a3 square.
    ///
    pub fn set_tile(&mut self, tile: &str, piece: String) {
        println!("Set tile {:?} to {:?}", tile, piece);
        let tile_index: usize = PieceMap::tile_index(tile);
        let mapped_piece: u8 = PieceMap::convert_to_piecemap(piece);
        self.map[tile_index] = mapped_piece;
    }

    /// Gets the index on a piecemap where a tile is. Takes standard notation eg. "c4" as an input and returns a usize of the index
    ///
    /// # Example
    ///
    /// ```
    /// let index: usize = tile_index("c4")
    /// ```
    /// Would get the index for c4, which is 19 (Zero based indexing).
    pub fn tile_index(tile: &str) -> usize {
        let letter_value: u32 = letter_to_number(tile.chars().nth(0).unwrap()).unwrap();
        let num_value: u32 = number_from_string(tile).unwrap();
        (((letter_value - 1) * 8) + (num_value - 1)) as usize
    }

    /// Returns the standard notation representation of a tile based on the index of an element in a piecemap.
    ///
    /// # Example
    ///
    /// ```
    /// let t: String = tile_from_index(7);
    /// ```
    /// Would get "a7".
    pub fn tile_from_index(index: i32) -> String {
        let letters: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let r: i32 = rank(index);
        let f: i32 = file(index);
        let first: &str = letters[(f - 1) as usize];
        let second: String = format!("{}", r);
        let tile: String = format!("{}{}", first, second);
        return tile;
    }

    /// Resets the piecemap to its default state.
    ///
    /// # Example
    ///
    /// ```
    /// let map: PieceMap = PieceMap::new();
    /// // Modify the map
    /// map.reset();
    /// ```
    ///
    pub fn reset(&mut self) {
        self.map = [
            0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
            0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000, 0b0000,
        ];
        self.check.set_active(false);
        self.current = true;
        self.attacked = Vec::new()
    }

    pub fn add_attacker(&mut self, index: i32) {
        self.attacked.push(index)
    }

    pub fn remove_attacker(&mut self, index: i32) {
        self.attacked.remove(
            self.attacked
                .iter()
                .position(|x: &i32| x == &index)
                .unwrap() as usize,
        );
    }

    pub fn manage_attacker(&mut self, index: i32) {
        if self.attacked.contains(&index) {
            self.attacked.remove(
                self.attacked
                    .iter()
                    .position(|x: &i32| x == &index)
                    .unwrap() as usize,
            );
            println!("removed {}", index)
        } else {
            self.attacked.push(index);
            println!("added {}", index)
        }
    }
}
