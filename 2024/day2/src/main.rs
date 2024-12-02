use std::fs::read_to_string;
fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let input_lines = read_lines("input.txt");

    let mut safe_records_count: u32 = 0;
    for line in input_lines {
        let numbers: Vec<u32> = line
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        match numbers[0].cmp(&numbers[1]) {
            std::cmp::Ordering::Less => {
                if numbers.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3) {
                    safe_records_count += 1;
                }
            }
            std::cmp::Ordering::Equal => {
                continue;
            }
            std::cmp::Ordering::Greater => {
                if numbers.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3) {
                    safe_records_count += 1;
                }
            }
        };
    }
    safe_records_count
}

fn part2() -> u32 {
    let input_lines = read_lines("input.txt");
    0
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
