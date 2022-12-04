use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

// my shapes
const X: u32 = 1;
const Y: u32 = 2;
const Z: u32 = 3;

// scores
const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOST: u32 = 0;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_total_score(lines: Lines<BufReader<File>>) -> u32 {
    lines
        .into_iter()
        .filter_map(|line| {
            if let Some(str) = line.ok() {
                let opponent: String = str.clone().chars().nth(0).unwrap().to_string();
                let me: String = str.chars().nth(2).unwrap().to_string();
                match (&*opponent, &*me) {
                    ("A", "X") => Some(X + DRAW), // rock, rock
                    ("A", "Y") => Some(Y + WIN),  // rock, paper
                    ("A", "Z") => Some(Z + LOST), // rock, scissor
                    ("B", "X") => Some(X + LOST), // paper, rock
                    ("B", "Y") => Some(Y + DRAW), // paper, paper
                    ("B", "Z") => Some(Z + WIN),  // paper, scissor
                    ("C", "X") => Some(X + WIN),  // scissor, rock
                    ("C", "Y") => Some(Y + LOST), // scissor, paper
                    ("C", "Z") => Some(Z + DRAW), // scissor, scissor
                    (&_, _) => Some(0),
                }
            } else {
                Some(0)
            }
        })
        .sum()
}

fn main() {
    if let Ok(lines) = read_lines("./example.txt") {
        assert_eq!(get_total_score(lines), 15);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        println!("{:?}", get_total_score(lines));
    }
}
