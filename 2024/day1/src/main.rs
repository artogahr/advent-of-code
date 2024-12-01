use std::{fs::read_to_string, u32};
fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> u32 {
    let input_lines = read_lines("input.txt");

    let mut left_side: Vec<u32> = Vec::new();
    let mut right_side: Vec<u32> = Vec::new();

    for line in input_lines {
        let (left, right) = line
            .split_once(' ')
            .map(|(l, r)| {
                (
                    l.trim().parse::<u32>().unwrap(),
                    r.trim().parse::<u32>().unwrap(),
                )
            })
            .unwrap();
        left_side.push(left);
        right_side.push(right);
    }
    left_side.sort();
    right_side.sort();

    let mut total: u32 = 0;
    for i in 0..left_side.len() {
        total += left_side[i].abs_diff(right_side[i]);
    }
    total
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
