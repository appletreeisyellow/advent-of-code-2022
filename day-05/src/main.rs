// "    [D]    " -> ['', 'D', '']
// "[Z] [M] [P]" -> ['Z', 'M', 'P']
fn parse_crates(line: &str) -> Vec<char> {
    line[1..line.len()]
        .chars()
        .step_by(4) // the letters are 4 char away from each other
        .map(|c| c)
        .collect::<Vec<char>>()
}

// " 1   2   3 " -> [1, 2, 3]
fn get_total_crates(line: &str) -> Vec<i32> {
    line.split(" ")
        .filter_map(|crate_str| {
            if crate_str == "" {
                None
            } else {
                Some(crate_str.parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<i32>>()
}

// " 1   2   3 " -> [[], [], ]
fn create_crate_stacks(line: &str) -> Vec<Vec<char>> {
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

fn get_final_crates(lines: Vec<&str>) -> &str {
    // println!("{:?}", lines);

    // find the index of the empty line
    let empty_line: usize = lines.iter().position(|&line: &&str| line == "").unwrap();

    // construct stacks of crates
    //   get the total crate num
    let crate_bottom: usize = empty_line - 1;
    let total_crates: Vec<i32> = get_total_crates(lines[crate_bottom]);

    println!("total_crates {:?}", total_crates);

    //   stack crates line by line from bottom to top
    let mut stacks: Vec<Vec<char>> = create_crate_stacks(lines[crate_bottom]);
    println!("{:?}", stacks);

    lines[0..crate_bottom]
        .iter()
        .rev()
        .for_each(|&line: &&str| {
            let crates: Vec<char> = parse_crates(line);
            println!("{:?}", crates);
        });

    // parse movement
    // move crates

    "ABC"
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_helper_functions() {
        assert_eq!(get_total_crates(" 1   2   3 "), vec![1, 2, 3]);
        assert_eq!(
            get_total_crates("1   2   3   4   5   6   7   8   9 "),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );

        assert_eq!(parse_crates("    [D]    "), vec![' ', 'D', ' ']);
        assert_eq!(parse_crates("[N] [C]    "), vec!['N', 'C', ' ']);
        assert_eq!(parse_crates("[Z] [M] [P]"), vec!['Z', 'M', 'P']);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(get_final_crates(EXAMPLE.lines().collect()), "CMZ");
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {:?}", get_final_crates(input));
}
