fn get_num_pairs(lines: Vec<&str>) -> i32 {
    lines
        .into_iter()
        .filter_map(|line: &str| is_fully_contain(line).then_some(1))
        .sum::<i32>()
}

fn is_fully_contain(line: &str) -> bool {
    let pairs: Vec<Vec<i32>> = line
        .split(',')
        .map(|range_str: &str| {
            let range_num: Vec<i32> = range_str
                .split('-')
                .map(|str: &str| str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            range_num
        })
        .collect::<Vec<_>>();

    // println!("{:?}", pairs);

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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_helper_functions() {
        assert_eq!(is_fully_contain("6-6,4-6"), true);
        assert_eq!(is_fully_contain("4-6,6-6"), true);
        assert_eq!(is_fully_contain("2-8,3-7"), true);
        assert_eq!(is_fully_contain("3-7,2-8"), true);
        assert_eq!(is_fully_contain("3-7,5-7"), true);
        assert_eq!(is_fully_contain("5-7,3-7"), true);
        assert_eq!(is_fully_contain("2-2,2-2"), true);
        assert_eq!(is_fully_contain("2-4,6-8"), false);
        assert_eq!(is_fully_contain("2-3,4-5"), false);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(get_num_pairs(EXAMPLE.lines().collect()), 2);
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {:?}", get_num_pairs(input));
}
