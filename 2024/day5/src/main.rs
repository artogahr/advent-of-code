use std::{collections::HashMap, fs::read_to_string, u32};
fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let mut sum = 0;
    let lines = read_lines("input.txt");
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        if line.chars().any(|ch| ch == '|') {
            let (left, right) = line
                .split_once("|")
                .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
                .unwrap();
            match rules.get_mut(&right) {
                Some(comes_after) => {
                    comes_after.push(left);
                }
                None => {
                    rules.insert(right, vec![left]);
                }
            }
        } else {
            let numbers: Vec<u32> = line
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            let mut is_valid = true;
            for (i, num) in numbers.iter().enumerate() {
                if let Some(num_comes_after) = rules.get(num) {
                    for j in i..numbers.len() {
                        if num_comes_after.iter().any(|num| num == &numbers[j]) {
                            is_valid = false;
                        }
                    }
                }
            }
            if is_valid {
                sum += numbers[numbers.len() / 2];
            }
        }
    }
    sum
}

fn part2() -> u32 {
    todo!()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
