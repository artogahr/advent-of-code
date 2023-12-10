use std::{env::current_exe, ops::Add};

fn parse_input() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|line: String| line.chars().collect())
        .collect()
}

fn part1(map: Vec<Vec<char>>) -> u32 {
    dbg!(&map);
    let mut first_pos: Position = Default::default();
    'outer: for (x, line) in map.iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            if char == 'S' {
                first_pos = Position { x: x, y: y };
                break 'outer;
            }
        }
    }
    dbg!(&first_pos);
    let (mut prev_pos, mut current_pos) = find_next_pos(&map, first_pos.clone(), first_pos);
    dbg!(&prev_pos, &current_pos);
    loop {
        println!("=============");
        println!("previous position is");
        dbg!(&prev_pos);
        println!("current position is");
        dbg!(&current_pos);
        (prev_pos, current_pos) = find_next_pos(&map, prev_pos, current_pos);
        println!("after changing: \nprevious position is");
        dbg!(&prev_pos);
        println!("current position is");
        dbg!(&current_pos);
        break;
    }
    todo!();
    0
}

#[derive(Debug, Default, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn find_next_pos(
    map: &Vec<Vec<char>>,
    prev_pos: Position,
    current_pos: Position,
) -> (Position, Position) {
    let mut current_pos = current_pos;
    match map[current_pos.x][current_pos.y] {
        '|' => {
            if current_pos.x > prev_pos.x {
                current_pos.x += 1;
            } else {
                current_pos.x -= 1;
            }
            return (prev_pos, current_pos);
        }
        '-' => {
            if current_pos.y > prev_pos.y {
                current_pos.y += 1;
            } else {
                current_pos.y -= 1;
            }
            return (prev_pos, current_pos);
        }
        'L' => {
            if current_pos.y == prev_pos.y {
                current_pos.y += 1;
            } else {
                current_pos.x -= 1;
            }
            return (prev_pos, current_pos);
        }
        'J' => {
            if current_pos.y == prev_pos.y {
                current_pos.y -= 1;
            } else {
                current_pos.x -= 1;
            }
            return (prev_pos, current_pos);
        }
        '7' => {
            if current_pos.y == prev_pos.y {
                current_pos.y -= 1;
            } else {
                current_pos.x += 1;
            }
            return (prev_pos, current_pos);
        }
        'F' => {
            if current_pos.y == prev_pos.y {
                current_pos.y += 1;
            } else {
                current_pos.x += 1;
            }
            return (prev_pos, current_pos);
        }
        _ => {
            dbg!(current_pos, prev_pos);
            panic!("dot reached");
        }
    }
}

fn part2(map: Vec<Vec<char>>) -> u32 {
    todo!();
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
