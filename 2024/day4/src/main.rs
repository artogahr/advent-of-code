use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("input.txt").unwrap());

    let word_search: Vec<Vec<char>> = f.lines().map(|l| l.unwrap().chars().collect()).collect();
    println!("Part 1: {}", part1(&word_search));
    println!("Part 2: {}", part2(&word_search));
}

#[allow(non_snake_case)]
fn part1(word_search: &Vec<Vec<char>>) -> u32 {
    let mut valid_XMAS_count: u32 = 0;

    for x in 0..word_search.len() as i32 {
        for y in 0..word_search[0].len() as i32 {
            if word_search[x as usize][y as usize] == 'X' {
                valid_XMAS_count += num_valid_XMAS(&word_search, x as i32, y as i32);
            }
        }
    }
    valid_XMAS_count
}

#[allow(non_snake_case)]
fn part2(word_search: &Vec<Vec<char>>) -> u32 {
    let mut valid_X_MAS_count: u32 = 0;

    for x in 0..word_search.len() as i32 {
        for y in 0..word_search[0].len() as i32 {
            if word_search[x as usize][y as usize] == 'A'
                && num_valid_X_MAS(&word_search, x as i32, y as i32)
            {
                valid_X_MAS_count += 1;
            }
        }
    }
    valid_X_MAS_count
}

#[allow(non_snake_case)]
fn num_valid_X_MAS(word_search: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let mut M_A_S_count: u32 = 0;
    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    for direction in directions {
        let x_new = x + direction.0;
        let y_new = y + direction.1;
        if x_new < 0
            || y_new < 0
            || x_new >= word_search.len() as i32
            || y_new >= word_search[0].len() as i32
        {
            continue;
        }
        if word_search[x_new as usize][y_new as usize] == 'M' {
            let x_new = x + direction.0 * -1;
            let y_new = y + direction.1 * -1;
            if x_new < 0
                || y_new < 0
                || x_new >= word_search.len() as i32
                || y_new >= word_search[0].len() as i32
            {
                continue;
            }
            if word_search[x_new as usize][y_new as usize] == 'S' {
                M_A_S_count += 1;
            }
        }
    }
    M_A_S_count == 2
}

#[allow(non_snake_case)]
fn num_valid_XMAS(word_search: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let mut valid_XMAS_count: u32 = 0;
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for direction in directions {
        let x_new = x + direction.0;
        let y_new = y + direction.1;
        if x_new < 0
            || y_new < 0
            || x_new >= word_search.len() as i32
            || y_new >= word_search[0].len() as i32
        {
            continue;
        }
        if word_search[x_new as usize][y_new as usize] == 'M' {
            let x_new = x_new + direction.0;
            let y_new = y_new + direction.1;
            if x_new < 0
                || y_new < 0
                || x_new >= word_search.len() as i32
                || y_new >= word_search[0].len() as i32
            {
                continue;
            }
            if word_search[x_new as usize][y_new as usize] == 'A' {
                let x_new = x_new + direction.0;
                let y_new = y_new + direction.1;
                if x_new < 0
                    || y_new < 0
                    || x_new >= word_search.len() as i32
                    || y_new >= word_search[0].len() as i32
                {
                    continue;
                }
                if word_search[x_new as usize][y_new as usize] == 'S' {
                    valid_XMAS_count += 1;
                }
            }
        }
    }
    valid_XMAS_count
}
