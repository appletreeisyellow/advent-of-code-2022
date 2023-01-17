use std::collections::HashMap;

struct Node {
    name: String,
    is_directory: bool,
    parent: Option<Node>,
    children: Option<HashMap<String, Node>>,
    size: i32,
}

fn parse_to_data_structure(lines: Vec<&str>) -> Vec<i32> {
    let graph: Node = Node {
        name: String::from("/"),
        is_directory: true,
        parent: None,
        children: None,
        size: 0,
    };
    let current_position: &Node = &graph;
    let mut is_forming_children: bool = false;

    lines.into_iter().for_each(|line| match &line[..4] {
        "$ cd" => {
            println!("cd");
            // go to a dir
            is_forming_children = false;
        }
        "$ ls" => {
            println!("ls");
            // forming children, do nothing for this line
        }
        "dir " => {
            println!("dir");
        }
        _ => {
            println!("{:?}", line);
            // record file name and size
            // split the line into vec of strings
            // str[0] is the size
            // str[1] is the name
            let file_str: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
            let file_node: Node = Node {
                name: String::from(file_str[1]),
                is_directory: false,
                parent: Some(Box::new(current_position)),
                children: None,
                size: file_str[0].parse::<i32>().unwrap(),
            }
        }
    });

    vec![1]
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_one() {
        parse_to_data_structure(EXAMPLE.lines().collect());
    }
}

fn main() {
    println!("Hello, world!");
}
