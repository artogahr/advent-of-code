use std::{env::current_exe, ops::Add};

use advent::new;

fn parse_input() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|line: String| line.chars().collect())
        .collect()
}

fn part1(map: Vec<Vec<char>>) -> u32 {
    let mut first_pos: Position = Default::default();
    let mut current_pos: Position = Default::default();
    'outer: for (x, line) in map.iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            if char == 'S' {
                first_pos = Position { x: x, y: y };
                current_pos = find_valid_move(&map, &first_pos);
                break 'outer;
            }
        }
    }
    let (mut prev_pos, mut current_pos) = find_next_pos(&map, first_pos, current_pos);
    let mut step_count = 0;
    loop {
        (prev_pos, current_pos) = find_next_pos(&map, prev_pos, current_pos);
        if map[current_pos.x][current_pos.y] == 'S' {
            println!("found s");
            break;
        } else {
            step_count += 1;
            continue;
        }
    }
    (step_count + 3) / 2
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
    let current_pos_clone = current_pos.clone();
    match map[current_pos.x][current_pos.y] {
        '|' => {
            if current_pos.x > prev_pos.x {
                current_pos.x += 1;
            } else {
                current_pos.x -= 1;
            }
            return (current_pos_clone, current_pos);
        }
        '-' => {
            if current_pos.y > prev_pos.y {
                current_pos.y += 1;
            } else {
                current_pos.y -= 1;
            }
            return (current_pos_clone, current_pos);
        }
        'L' => {
            if current_pos.y == prev_pos.y {
                current_pos.y += 1;
            } else {
                current_pos.x -= 1;
            }
            return (current_pos_clone, current_pos);
        }
        'J' => {
            if current_pos.y == prev_pos.y {
                current_pos.y -= 1;
            } else {
                current_pos.x -= 1;
            }
            return (current_pos_clone, current_pos);
        }
        '7' => {
            if current_pos.y == prev_pos.y {
                current_pos.y -= 1;
            } else {
                current_pos.x += 1;
            }
            return (current_pos_clone, current_pos);
        }
        'F' => {
            if current_pos.y == prev_pos.y {
                current_pos.y += 1;
            } else {
                current_pos.x += 1;
            }
            return (current_pos_clone, current_pos);
        }
        _ => {
            dbg!(current_pos, prev_pos);
            panic!("dot reached");
        }
    }
}

fn find_valid_move(map: &Vec<Vec<char>>, index: &Position) -> Position {
    if index.x > 0 {
        match map[index.x - 1][index.y] {
            '|' | '7' | 'F' => {
                return Position {
                    x: index.x - 1,
                    y: index.y,
                }
            }
            _ => {}
        }
    }
    if index.x < map[0].len() {
        match map[index.x + 1][index.y] {
            '|' | 'J' | 'L' => {
                return Position {
                    x: index.x + 1,
                    y: index.y,
                }
            }
            _ => {}
        }
    }
    if index.y < map.len() {
        match map[index.x][index.y + 1] {
            '-' | 'J' | '7' => {
                return Position {
                    x: index.x,
                    y: index.y + 1,
                }
            }
            _ => {}
        }
    }
    if index.y > 0 {
        match map[index.x][index.y - 1] {
            '-' | 'L' | 'F' => {
                return Position {
                    x: index.x,
                    y: index.y - 1,
                }
            }
            _ => {}
        }
    }
    panic!("Couldn't find valid position for {} {}", index.x, index.y);
}

fn part2(map: Vec<Vec<char>>) -> u32 {
    let mut first_pos: Position = Default::default();
    let mut current_pos: Position = Default::default();
    let mut new_map: Vec<Vec<char>> = map.clone();
    for line in new_map.iter_mut() {
        for ch in line.iter_mut() {
            *ch = ' ';
        }
    }
    //dbg!(&new_map);
    'outer: for (x, line) in map.iter().enumerate() {
        for (y, &char) in line.iter().enumerate() {
            if char == 'S' {
                first_pos = Position { x: x, y: y };
                current_pos = find_valid_move(&map, &first_pos);
                break 'outer;
            }
        }
    }
    let (mut prev_pos, mut current_pos) = find_next_pos(&map, first_pos, current_pos);
    let mut step_count = 0;
    new_map[prev_pos.x][prev_pos.y] = '*';
    new_map[current_pos.x][current_pos.y] = '*';
    loop {
        (prev_pos, current_pos) = find_next_pos(&map, prev_pos, current_pos);
        match map[current_pos.x][current_pos.y] {
            'S' => {
                new_map[current_pos.x][current_pos.y] = '*';
                break;
            }
            'F' | '7' | 'L' | 'J' | '|' => {
                new_map[current_pos.x][current_pos.y] = '*';
                continue;
            }
            '-' => {
                new_map[current_pos.x][current_pos.y] = '-';
            }
            _ => {
                new_map[current_pos.x][current_pos.y] = ' ';
                continue;
            }
        }
    }
    for (x, line) in map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            print!("{ch}");
        }
        print!("\n");
    }
    for (x, line) in new_map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            print!("{ch}");
        }
        print!("\n");
    }
    let mut num_inside: u32 = 0;
    for (x, line) in new_map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            if *ch == ' ' {
                let num_walls = line[y..].iter().filter(|f| **f == '*').count();
                let straights = line[y..].iter().filter(|f| **f == '-').count();
                if num_walls % 2 != 0 && straights % 2 == 0 {
                    //dbg!(&map[x], &x, &y, &map[x][y], num_walls);
                    num_inside += 1;
                    //dbg!(&num_inside);
                    print!("I");
                } else {
                    print!("O");
                }
            } else {
                print!("{ch}");
            }
        }
        print!("\n");
    }

    num_inside
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
