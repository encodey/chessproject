use crate::core::piecemap::PieceMap;

/// Takes a u32 input and returns the value in binary, still under u32 representation.
pub fn decimal_to_binary(num: u32) -> u32 {
    if num == 0 {
        return 0;
    } else {
        return num % 2 + 10 * decimal_to_binary(num / 2);
    }
}

/// Takes a letter and converts it to its numerical position in the alphabet.
pub fn letter_to_number(letter: char) -> Option<u32> {
    if letter.is_ascii_alphabetic() {
        let lowercase_letter: char = letter.to_ascii_lowercase();
        Some((lowercase_letter as u32) - ('a' as u32) + 1)
    } else {
        None
    }
}

/// Takes a string input and extracts and returns the first number it finds.
pub fn number_from_string(input: &str) -> Option<u32> {
    let number_str: String = input
        .chars()
        .filter(|c: &char| c.is_numeric())
        .collect::<String>();
    number_str.parse::<u32>().ok()
}

/// Pads a u32 with leading zeros and returns it as a string.
pub fn padded(number: u32) -> String {
    format!("{:04}", number)
}

///
pub fn get_piece(main_string: &str, substring: &str, start_index: usize) -> String {
    let next_four_chars: &str =
        &main_string[start_index + substring.len()..start_index + substring.len() + 4];
    return next_four_chars.to_string();
}

pub fn file(index: i32) -> i32 {
    index / 8 + 1
}

pub fn rank(index: i32) -> i32 {
    (index - ((file(index) - 1) * 8)) + 1
}

pub fn is_diff_color(map: &std::sync::MutexGuard<'_, PieceMap>, i1: i32, i2: i32) -> bool {
    let a: Option<char> = PieceMap::convert_from_piecemap(map.map[i1 as usize])
        .chars()
        .nth(0);
    let b: Option<char> = PieceMap::convert_from_piecemap(map.map[i2 as usize])
        .chars()
        .nth(0);
    a != b
}
