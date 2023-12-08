use indexmap::IndexMap;
fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn part1(input: Vec<String>) -> i64 {
    let mut nodes: IndexMap<String, Node> = Default::default();
    let directions = input.iter().nth(0).unwrap();
    let mut current_node: String = "AAA".to_string();
    for node_line in &input[2..] {
        current_node = node_line.chars().take(3).collect();
        let left: &str = &node_line[7..10];
        let right = &node_line[12..15];
        nodes.insert(
            current_node.clone(),
            Node {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }
    let mut i: usize = 0;
    //dbg!(&nodes);
    current_node = "AAA".to_string();
    loop {
        match directions
            .chars()
            .nth(i.rem_euclid(directions.len()))
            .unwrap()
        {
            'R' => {
                current_node = nodes.get(&current_node).unwrap().right.clone();
                if current_node == "ZZZ" {
                    break;
                } else {
                    i += 1;
                    continue;
                }
            }
            'L' => {
                current_node = nodes.get(&current_node).unwrap().left.clone();
                if current_node == "ZZZ" {
                    break;
                } else {
                    i += 1;
                    continue;
                }
            }
            _ => continue,
        }
    }
    i as i64 + 1
}

fn part2(_input: Vec<String>) -> i64 {
    let mut nodes: IndexMap<String, Node> = Default::default();
    let directions = input.iter().nth(0).unwrap();
    let mut current_node: String = "AAA".to_string();
    for node_line in &input[2..] {
        current_node = node_line.chars().take(3).collect();
        let left: &str = &node_line[7..10];
        let right = &node_line[12..15];
        nodes.insert(
            current_node.clone(),
            Node {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }
    let mut i: usize = 0;
    //dbg!(&nodes);
    current_node = "AAA".to_string();
    loop {
        match directions
            .chars()
            .nth(i.rem_euclid(directions.len()))
            .unwrap()
        {
            'R' => {
                current_node = nodes.get(&current_node).unwrap().right.clone();
                if current_node == "ZZZ" {
                    break;
                } else {
                    i += 1;
                    continue;
                }
            }
            'L' => {
                current_node = nodes.get(&current_node).unwrap().left.clone();
                if current_node == "ZZZ" {
                    break;
                } else {
                    i += 1;
                    continue;
                }
            }
            _ => continue,
        }
    }
    i as i64 + 1
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
