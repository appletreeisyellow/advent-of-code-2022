// calories cannot be negative, so i used unsigned u16
fn find_max_sum(calories: &Vec<Vec<u16>>) -> Option<u16> {
    calories.into_iter().map(|v| v.iter().sum::<u16>()).max()
}

fn main() {
    let example: Vec<Vec<u16>> = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];
    assert_eq!(Some(24_000), find_max_sum(&example));
}
