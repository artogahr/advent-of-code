use core::num;
use std::fs::read_to_string;
fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u128 {
    let equations = read_lines("input.txt");
    let mut total_calibration_result = 0;
    let operations = vec!['+', '*'];

    for equation in equations {
        let (test_value, numbers) = equation.split_once(":").unwrap();
        let mut numbers: Vec<u128> = numbers
            .trim()
            .split(" ")
            .map(|num| num.trim().parse().unwrap())
            .collect();
        for variation in generate_variations(&operations, numbers.len() - 1) {
            if eval(&mut numbers, variation.clone()) == test_value.trim().parse().unwrap() {
                total_calibration_result += test_value.trim().parse::<u128>().unwrap();
                break;
            }
        }
    }
    total_calibration_result
}

fn generate_variations(chars: &[char], size: usize) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    fn generate_recursive(
        chars: &[char],
        current: Vec<char>,
        size: usize,
        result: &mut Vec<Vec<char>>,
    ) {
        if current.len() == size {
            result.push(current);
            return;
        }

        for &c in chars {
            let mut new_vec = current.clone();
            new_vec.push(c);
            generate_recursive(chars, new_vec, size, result);
        }
    }

    generate_recursive(chars, Vec::new(), size, &mut result);
    result
}

fn eval(numbers: &mut Vec<u128>, operations: Vec<char>) -> u128 {
    let mut end_result = 0;
    end_result += numbers[0];
    for i in 0..numbers.len() - 1 {
        match operations[i] {
            '+' => end_result += numbers[i + 1],
            '*' => end_result *= numbers[i + 1],
            '|' => {
                end_result = (end_result.to_string() + &numbers[i + 1].to_string())
                    .parse()
                    .unwrap();
            }
            _ => {}
        }
    }
    end_result
}

fn part2() -> u128 {
    let equations = read_lines("input.txt");
    let mut total_calibration_result = 0;
    let operations = vec!['+', '*', '|'];

    for equation in equations {
        let (test_value, numbers) = equation.split_once(":").unwrap();
        let mut numbers: Vec<u128> = numbers
            .trim()
            .split(" ")
            .map(|num| num.trim().parse().unwrap())
            .collect();
        for variation in generate_variations(&operations, numbers.len() - 1) {
            if eval(&mut numbers, variation.clone()) == test_value.trim().parse().unwrap() {
                total_calibration_result += test_value.trim().parse::<u128>().unwrap();
                break;
            }
        }
    }
    total_calibration_result
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
