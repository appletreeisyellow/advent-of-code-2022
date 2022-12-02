use std::fs;

// This is a helper function to convert the input string
// into the data structure we want
fn convert(str: &str) -> Vec<Vec<u32>> {
    str.split("\n\n")
        .map(|inner_str: &str| {
            inner_str
                .split("\n")
                .map(|s: &str| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

// calories cannot be negative, so i used unsigned u32
fn find_max_sum(calories: &Vec<Vec<u32>>) -> Option<u32> {
    calories.into_iter().map(|v| v.iter().sum::<u32>()).max()
}

fn find_top_three_sum(calories: &Vec<Vec<u32>>) -> Option<u32> {
    let mut sums: Vec<u32> = calories
        .into_iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>();

    sums.sort_by(|a: &u32, b: &u32| b.cmp(a));

    if sums.len() > 3 {
        Some(sums.into_iter().take(3).sum())
    } else {
        None
    }
}

fn main() {
    assert_eq!(convert("100\n200\n\n300"), vec![vec![100, 200], vec![300]]);

    let example: Vec<Vec<u32>> = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];
    assert_eq!(find_max_sum(&example), Some(24_000));
    assert_eq!(find_top_three_sum(&example), Some(45_000));

    let file_path = "./input.txt";
    let input_str = fs::read_to_string(file_path).expect("should have been able to read the file");
    let converted_input = convert(&input_str);
    println!("max sum -> {:?}", find_max_sum(&converted_input));
    println!(
        "top three sum -> {:?}",
        find_top_three_sum(&converted_input)
    );
}
