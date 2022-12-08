fn get_num_pairs(lines: Vec<&str>) -> i32 {
    lines
        .into_iter()
        .filter_map(|line: &str| is_fully_overlap(line).then_some(1))
        .sum::<i32>()
}

fn get_num_pairs_2(lines: Vec<&str>) -> i32 {
    lines
        .into_iter()
        .filter_map(|line: &str| is_overlap(line).then_some(1))
        .sum::<i32>()
}

// "1-2,3-4" -> [[1, 2], [3, 4]]
fn convert_to_num(line: &str) -> Vec<Vec<i32>> {
    line.split(',')
        .map(|range_str: &str| {
            let range_num: Vec<i32> = range_str
                .split('-')
                .map(|str: &str| str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            range_num
        })
        .collect::<Vec<_>>()
}

fn is_fully_overlap(line: &str) -> bool {
    let pairs: Vec<Vec<i32>> = convert_to_num(line);

    // pairs is like [[1, 2], [3, 4]]
    let (left1, left2): (i32, i32) = (pairs[0][0], pairs[1][0]);
    let (right1, right2): (i32, i32) = (pairs[0][1], pairs[1][1]);

    if left1 < left2 {
        right1 >= right2
    } else if left1 > left2 {
        right1 <= right2
    } else {
        true
    }
}

fn is_overlap(line: &str) -> bool {
    let pairs: Vec<Vec<i32>> = convert_to_num(line);

    // pairs is like [[1, 2], [3, 4]]
    let (left1, left2): (i32, i32) = (pairs[0][0], pairs[1][0]);
    let (right1, right2): (i32, i32) = (pairs[0][1], pairs[1][1]);

    if left1 < left2 {
        left2 <= right1
    } else if left1 > left2 {
        right2 >= left1
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_helper_functions() {
        assert_eq!(is_fully_overlap("6-6,4-6"), true);
        assert_eq!(is_fully_overlap("2-8,3-7"), true);
        assert_eq!(is_fully_overlap("3-7,5-7"), true);
        assert_eq!(is_fully_overlap("2-2,2-2"), true);
        assert_eq!(is_fully_overlap("2-4,6-8"), false);
        assert_eq!(is_fully_overlap("2-3,4-5"), false);

        assert_eq!(is_overlap("6-6,4-6"), true);
        assert_eq!(is_overlap("2-8,3-7"), true);
        assert_eq!(is_overlap("3-7,5-7"), true);
        assert_eq!(is_overlap("2-6,4-8"), true);
        assert_eq!(is_overlap("5-7,7-9"), true);
        assert_eq!(is_overlap("2-8,3-7"), true);
        assert_eq!(is_overlap("2-2,2-2"), true);
        assert_eq!(is_overlap("2-4,6-8"), false);
        assert_eq!(is_overlap("2-3,4-5"), false);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(get_num_pairs(EXAMPLE.lines().collect()), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_num_pairs_2(EXAMPLE.lines().collect()), 4);
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {:?}", get_num_pairs(input.clone()));
    println!("Part 2: {:?}", get_num_pairs_2(input));
}
