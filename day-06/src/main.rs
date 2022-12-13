use std::collections::{HashSet, VecDeque};

fn helper(str: &str, distinct_char: usize) -> usize {
    let mut queue: VecDeque<char> = VecDeque::new();

    if let Some(idx) = str.chars().enumerate().find_map(|(i, c): (usize, char)| {
        if queue.len() < distinct_char {
            queue.push_back(c);
            None
        } else {
            queue.pop_front();
            queue.push_back(c);

            let r: HashSet<char> = queue.clone().into_iter().collect::<HashSet<char>>();
            if r.len() == distinct_char {
                Some(i)
            } else {
                None
            }
        }
    }) {
        idx + 1
    } else {
        0
    }
}

fn get_first_marker(str: &str) -> usize {
    helper(str, 4)
}

fn get_first_marker_2(str: &str) -> usize {
    helper(str, 14)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(get_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(get_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(get_first_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(get_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(get_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_first_marker_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(get_first_marker_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(get_first_marker_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(get_first_marker_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(get_first_marker_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!(
        "Part 1: {:?}",
        get_first_marker(input.clone().get(0).unwrap())
    );
    println!("Part 2: {:?}", get_first_marker_2(input.get(0).unwrap()));
}
