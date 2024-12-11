use std::{collections::HashMap, fs};

fn main() {
    let mut found_dividers: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut found_multipliers: HashMap<u64, u64> = HashMap::new();
    part1(&mut found_dividers, &mut found_multipliers);
    part2(&mut found_dividers, &mut found_multipliers);
}

fn part1(found_dividers: &mut HashMap<u64, (u64, u64)>, found_multipliers: &mut HashMap<u64, u64>) {
    let mut stones: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|str| str.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink(&stones, found_dividers, found_multipliers);
    }
    println!("Part 1: {0}", stones.len());
}

fn part2(found_dividers: &mut HashMap<u64, (u64, u64)>, found_multipliers: &mut HashMap<u64, u64>) {
    let mut stones: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|str| str.parse().unwrap())
        .collect();

    for i in 0..75 {
        println!("{i}");
        stones = blink(&stones, found_dividers, found_multipliers);
    }
    println!("Part 1: {0}", stones.len());
}

#[inline(always)]
fn blink(
    stones: &Vec<u64>,
    found_dividers: &mut HashMap<u64, (u64, u64)>,
    found_multipliers: &mut HashMap<u64, u64>,
) -> Vec<u64> {
    let mut new_stones: Vec<u64> = Vec::new();
    for stone in stones {
        match stone {
            0 => {
                new_stones.push(1);
            }
            _ if stone.to_string().len() % 2 == 0 => {
                let (stone1, stone2) = found_dividers.entry(*stone).or_insert({
                    let midpoint = stone.to_string().len() / 2;
                    let (stone1, stone2) = (stone / midpoint as u64, stone % midpoint as u64);
                    (stone1, stone2)
                });

                new_stones.push(*stone1);
                new_stones.push(*stone2);
            }
            _ => {
                new_stones.push(*found_multipliers.entry(*stone).or_insert(stone * 2024));
            }
        }
    }
    new_stones
}
