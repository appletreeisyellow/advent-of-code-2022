use std::collections::{HashSet, VecDeque};

fn get_first_marker(str: &str) -> usize {
    let mut queue: VecDeque<char> = VecDeque::new();

    if let Some(idx) = str.chars().enumerate().find_map(|(i, c): (usize, char)| {
        if queue.len() < 4 {
            queue.push_back(c);
            None
        } else {
            queue.pop_front();
            queue.push_back(c);

            let r: HashSet<char> = queue.clone().into_iter().collect::<HashSet<char>>();
            if r.len() == 4 {
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
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    println!("Part 1: {:?}", get_first_marker(input.get(0).unwrap()));
}
