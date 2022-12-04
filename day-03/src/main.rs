use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Convert the ascii value in decimal to priority value
//   - Lowercase item types a through z have priorities 1 through 26
//   - Uppercase item types A through Z have priorities 27 through 52
fn convert_to_priority(v: u32) -> u32 {
    if ('a' as u32) <= v && v <= ('z' as u32) {
        v - 96 // 'a' is 97
    } else if ('A' as u32) <= v && v <= ('Z' as u32) {
        v - 38 // 'A' is 65
    } else {
        0
    }
}

// find the common character between str1 and str2
// e.g. "abc", "bdd" -> "b"
fn find_common_char(str1: &str, str2: &str) -> char {
    let hashset1: HashSet<_> = HashSet::from_iter(
        str1.split("")
            .filter_map(|char| if char != "" { Some(char) } else { None })
            .collect::<Vec<&str>>(),
    );

    let hashset2: HashSet<_> = HashSet::from_iter(
        str2.split("")
            .filter_map(|char| if char != "" { Some(char) } else { None })
            .collect::<Vec<&str>>(),
    );

    let intersection = hashset1.intersection(&hashset2);

    // should just have one element in intersection
    if let Some(common_str) = intersection.last() {
        common_str.chars().nth(0).unwrap()
    } else {
        ' '
    }
}

fn get_priority_sum(lines: Lines<BufReader<File>>) -> u32 {
    lines
        .into_iter()
        .filter_map(|line| {
            if let Some(row) = line.ok() {
                let mid = row.len() / 2;
                let first_compartment = &row[..mid];
                let second_compartment = &row[mid..];
                let common_char = find_common_char(first_compartment, second_compartment);
                Some(convert_to_priority(common_char as u32))
            } else {
                Some(0)
            }
        })
        .sum::<u32>()
}

fn main() {
    // test helper functions
    assert_eq!(convert_to_priority('a' as u32), 1);
    assert_eq!(convert_to_priority('b' as u32), 2);
    assert_eq!(convert_to_priority('z' as u32), 26);
    assert_eq!(convert_to_priority('A' as u32), 27);
    assert_eq!(convert_to_priority('Z' as u32), 52);
    assert_eq!(find_common_char("abc", "bdd"), 'b');
    assert_eq!(find_common_char("abcds", "pesr"), 's');

    if let Ok(lines) = read_lines("./example.txt") {
        assert_eq!(get_priority_sum(lines), 157);
    }
    if let Ok(lines) = read_lines("./input.txt") {
        println!("{:?}", get_priority_sum(lines));
    }
}
