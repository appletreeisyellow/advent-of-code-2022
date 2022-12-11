// "    [D]    " -> ['', 'D', '']
// "[Z] [M] [P]" -> ['Z', 'M', 'P']
fn parse_crates(line: &str) -> Vec<char> {
    line[1..line.len()]
        .chars()
        .step_by(4) // the letters are 4 char away from each other
        .map(|c| c)
        .collect::<Vec<char>>()
}

// " 1   2   3 " -> [[], [], ]
fn create_empty_stacks(line: &str) -> Vec<Vec<char>> {
    line.split(" ")
        .filter_map(
            |crate_str| {
                if crate_str == "" {
                    None
                } else {
                    Some(vec![])
                }
            },
        )
        .collect::<Vec<Vec<char>>>()
}

fn construct_stacks(lines: &Vec<&str>, crate_bottom: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = create_empty_stacks(lines[crate_bottom]);
    lines[0..crate_bottom]
        .iter()
        .rev()
        .for_each(|&line: &&str| {
            let crates: Vec<char> = parse_crates(line);
            crates.iter().enumerate().for_each(|(i, &c)| {
                if c != ' ' {
                    stacks[i].push(c)
                }
            });
        });
    stacks
}

// "move 3 from 2 to 1" -> (3, 2, 1)
fn parse_movement(line: &str) -> (i32, i32, i32) {
    let res: Vec<i32> = line
        .split(" ")
        .filter_map(|str: &str| match str.parse::<i32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<i32>>();

    (res[0], res[1], res[2])
}

fn get_final_crates(lines: Vec<&str>) -> String {
    // find the index of the empty line
    let empty_line: usize = lines.iter().position(|&line: &&str| line == "").unwrap();

    // construct stacks of crates
    let mut stacks: Vec<Vec<char>> = construct_stacks(&lines, empty_line - 1);

    let movement_start: usize = empty_line + 1;
    lines[movement_start..lines.len()]
        // lines[movement_start..20]
        .into_iter()
        .for_each(|&line: &&str| {
            // parse movement
            let (num_crate, source, destination): (i32, i32, i32) = parse_movement(line);

            // move crate
            if num_crate > 0 {
                (0..num_crate).for_each(|_| {
                    // pop from source, push to destination
                    if let Some(crate_to_move) = stacks[source as usize - 1].pop() {
                        stacks[destination as usize - 1].push(crate_to_move);
                    }
                })
            }
        });

    stacks
        .into_iter()
        .filter_map(|stack: Vec<char>| stack.clone().pop())
        .collect::<String>()
}

fn get_final_crates_2(lines: Vec<&str>) -> String {
    "A".to_string()
}
#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_helper_functions() {
        assert_eq!(parse_crates("    [D]    "), vec![' ', 'D', ' ']);
        assert_eq!(parse_crates("[N] [C]    "), vec!['N', 'C', ' ']);
        assert_eq!(parse_crates("[Z] [M] [P]"), vec!['Z', 'M', 'P']);

        assert_eq!(parse_movement("move 3 from 2 to 1"), (3, 2, 1));
        assert_eq!(parse_movement("move 1 from 2 to 1"), (1, 2, 1));
        assert_eq!(parse_movement("move 13 from 2 to 1"), (13, 2, 1));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(get_final_crates(EXAMPLE.lines().collect()), "CMZ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_final_crates_2(EXAMPLE.lines().collect()), "MCD");
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {:?}", get_final_crates(input));
}
