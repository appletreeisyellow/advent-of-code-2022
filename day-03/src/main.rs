// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Lines, Result};
// use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

// Convert the ascii value in decimal to priority value
//   - Lowercase item types a through z have priorities 1 through 26
//   - Uppercase item types A through Z have priorities 27 through 52
fn convert_to_priority(v: u8) -> u8 {
    if ('a' as u8) <= v && v <= ('z' as u8) {
        v - 96 // 'a' is 97
    } else if ('A' as u8) <= v && v <= ('Z' as u8) {
        v - 38 // 'A' is 65
    } else {
        0
    }
}

fn main() {
    assert_eq!(convert_to_priority('a' as u8), 1);
    assert_eq!(convert_to_priority('b' as u8), 2);
    assert_eq!(convert_to_priority('z' as u8), 26);
    assert_eq!(convert_to_priority('A' as u8), 27);
    assert_eq!(convert_to_priority('Z' as u8), 52);
}
