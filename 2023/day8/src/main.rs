use indexmap::IndexMap;
use std::collections::HashMap;
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
    let mut current_node: String;
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

fn part2(lines: Vec<String>) -> u64 {
    let map = get_map(&lines[2..]);
    let totals = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(String::to_owned)
        .map(|start| steps(&lines[0], start, &map))
        .collect::<Vec<u64>>();
    let answer = totals.iter().fold(1, |x, steps| lcm(x, *steps));
    answer
}

fn steps(direction_str: &str, start: String, map: &HashMap<String, Elements>) -> u64 {
    let mut node = start;
    let mut step_count = 0;
    for direction in direction_str.chars().map(Direction::from_char).cycle() {
        if node.ends_with('Z') {
            return step_count;
        } else {
            node = map.get(&node).unwrap().get(&direction).to_string();
            step_count += 1;
        }
    }
    0
}

fn get_map(lines: &[String]) -> HashMap<String, Elements> {
    let mut map = HashMap::new();
    for line in lines {
        let (key, nodes) = line.split_once('=').unwrap();
        let key = key.trim().to_string();
        let (left, right) = nodes.trim().split_once(',').unwrap();
        let left = left.trim().strip_prefix('(').unwrap().to_string();
        let right = right.trim().strip_suffix(')').unwrap().to_string();
        map.insert(key, Elements::new(left, right));
    }
    map
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("Unknown direction '{c}'"),
        }
    }
}

struct Elements {
    left: String,
    right: String,
}

impl Elements {
    fn new(left: String, right: String) -> Self {
        Elements { left, right }
    }

    fn get(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
