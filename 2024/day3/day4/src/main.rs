use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("input.txt").unwrap());

    let word_search: Vec<Vec<char>> = f.lines().map(|l| l.unwrap().chars().collect()).collect();

    let mut valid_XMAS_count: u32 = 0;

    for x in 0..word_search.len() {
        for y in 0..word_search[0].len() {
            if word_search[x][y] == 'X' {}
        }
    }
    valid_XMAS_count += num_valid_XMAS(&word_search, 0, 0);
}

fn num_valid_XMAS(word_search: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
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

    let x: usize = 0;
    let y: usize = 0;
    for direction in directions {
        let x_dir: usize = x + usize::try_from(direction.0).unwrap_or(0);
        let y_dir: usize = y + usize::try_from(direction.1).unwrap_or(0);
        println!("{} {} {x}, {y}, {x_dir}, {y_dir}", direction.0, direction.1);
    }

    valid_XMAS_count
}
