use std::collections::HashSet;

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

fn convert_to_hashset(str: &str) -> HashSet<&str> {
    HashSet::from_iter(
        str.split("")
            .filter_map(|char: &str| if char != "" { Some(char) } else { None })
            .collect::<Vec<&str>>(),
    )
}

// find the common character between strs in a vec
// e.g. ["abc", "bdd"] -> "b"
fn find_common_char(strs: Vec<&str>) -> char {
    let mut intersection_result: HashSet<&str> = convert_to_hashset(strs.clone()[0]);
    strs.clone().into_iter().for_each(|str: &str| {
        let hashset: HashSet<&str> = convert_to_hashset(str);
        intersection_result = intersection_result
            .intersection(&hashset)
            .copied()
            .collect();
    });

    if let Some(intersection_str) = intersection_result.into_iter().nth(0) {
        intersection_str.chars().nth(0).unwrap()
    } else {
        ' '
    }
}

fn get_priority_sum(lines: Vec<&str>) -> u32 {
    lines
        .into_iter()
        .filter_map(|line| {
            let mid = line.len() / 2;
            let first_compartment = &line[..mid];
            let second_compartment = &line[mid..];
            let common_char = find_common_char(vec![first_compartment, second_compartment]);
            Some(convert_to_priority(common_char as u32))
        })
        .sum::<u32>()
}

fn get_priority_sum_2(lines: Vec<&str>) -> u32 {
    lines
        .chunks(3)
        .map(|group: &[&str]| {
            let transformed: Vec<&str> = group.into_iter().map(|&s| s.into()).collect();
            let common_char: char = find_common_char(transformed);
            convert_to_priority(common_char as u32)
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_helper_functions() {
        assert_eq!(convert_to_priority('a' as u32), 1);
        assert_eq!(convert_to_priority('b' as u32), 2);
        assert_eq!(convert_to_priority('z' as u32), 26);
        assert_eq!(convert_to_priority('A' as u32), 27);
        assert_eq!(convert_to_priority('Z' as u32), 52);
        assert_eq!(find_common_char(vec!["abc", "bdd"]), 'b');
        assert_eq!(find_common_char(vec!["abcds", "pesr"]), 's');
    }

    #[test]
    fn test_part_one() {
        assert_eq!(get_priority_sum(EXAMPLE.lines().collect()), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_priority_sum_2(EXAMPLE.lines().collect()), 70);
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {:?}", get_priority_sum(input.clone()));
    println!("Part 2: {:?}", get_priority_sum_2(input));
}
