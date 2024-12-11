use std::fs;

fn main() {
    part1();
}

fn part1() {
    let mut stones: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|str| str.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink(&stones);
    }
    println!("Part 1: {0}", stones.len());
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::new();
    for stone in stones {
        match stone {
            0 => {
                new_stones.push(1);
            }
            _ if stone.to_string().chars().count() % 2 == 0 => {
                let stone = stone.to_string();
                let (stone1, stone2) = stone.split_at(stone.to_string().len() / 2);

                new_stones.push(stone1.parse().unwrap());
                new_stones.push(stone2.parse().unwrap());
            }
            _ => {
                new_stones.push(stone * 2024);
            }
        }
    }
    new_stones
}
