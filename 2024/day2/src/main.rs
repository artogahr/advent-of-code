use std::fs::read_to_string;
fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let records = read_lines("input.txt");

    let mut safe_records_count: u32 = 0;
    for record in records {
        let record: Vec<u32> = record
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        if is_valid_record(&record) {
            safe_records_count += 1;
        }
    }
    safe_records_count
}

fn part2() -> u32 {
    let records = read_lines("input.txt");

    let mut safe_records_count: u32 = 0;

    for record in records {
        let record: Vec<u32> = record
            .split(' ')
            .map(|level| level.parse::<u32>().unwrap())
            .collect();

        if is_valid_record(&record) {
            safe_records_count += 1;
        } else {
            'outer: for i in 0..record.len() {
                let mut new_record: Vec<u32> = Vec::new();
                for j in 0..record.len() {
                    if i != j {
                        new_record.push(record[j]);
                    }
                }
                if is_valid_record(&new_record) {
                    safe_records_count += 1;
                    break 'outer;
                }
            }
        }
    }

    safe_records_count
}

fn is_valid_record(record: &Vec<u32>) -> bool {
    match record[0].cmp(&record[1]) {
        std::cmp::Ordering::Less => {
            if record.windows(2).all(|w| w[0] < w[1] && w[1] - w[0] <= 3) {
                true
            } else {
                false
            }
        }
        std::cmp::Ordering::Equal => false,
        std::cmp::Ordering::Greater => {
            if record.windows(2).all(|w| w[0] > w[1] && w[0] - w[1] <= 3) {
                true
            } else {
                false
            }
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
