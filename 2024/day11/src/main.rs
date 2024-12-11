use std::{collections::HashMap, fs};

fn main() {
    let mut found_dividers: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut found_multipliers: HashMap<u64, u64> = HashMap::new();
    part1(&mut found_dividers, &mut found_multipliers);
    part2(&mut found_dividers, &mut found_multipliers);
}

fn part1(found_dividers: &mut HashMap<u64, (u64, u64)>, found_multipliers: &mut HashMap<u64, u64>) {
    let stones: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|str| str.parse().unwrap())
        .collect();

    let mut stones_map: HashMap<u64, usize> = HashMap::new();
    stones.iter().for_each(|stone| {
        stones_map.insert(*stone, 1);
    });

    for _ in 0..25 {
        stones_map = blink(&stones_map, found_dividers, found_multipliers);
    }
    let result = stones_map
        .iter()
        .fold(0, |sum, (_number, count)| sum + count);

    println!("Part 1: {result}");
}

fn part2(found_dividers: &mut HashMap<u64, (u64, u64)>, found_multipliers: &mut HashMap<u64, u64>) {
    let stones: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|str| str.parse().unwrap())
        .collect();

    let mut stones_map: HashMap<u64, usize> = HashMap::new();
    stones.iter().for_each(|stone| {
        stones_map.insert(*stone, 1);
    });

    for _ in 0..75 {
        stones_map = blink(&stones_map, found_dividers, found_multipliers);
    }
    let result = stones_map
        .iter()
        .fold(0, |sum, (_number, count)| sum + count);

    println!("Part 2: {result}");
}

#[inline(always)]
fn blink(
    stones: &HashMap<u64, usize>,
    found_dividers: &mut HashMap<u64, (u64, u64)>,
    found_multipliers: &mut HashMap<u64, u64>,
) -> HashMap<u64, usize> {
    let mut new_stones: HashMap<u64, usize> = HashMap::new();
    for (stone, count) in stones.clone() {
        match stone {
            0 => {
                *new_stones.entry(1).or_insert(0) += count;
            }
            _ if stone.to_string().len() % 2 == 0 => {
                let (stone1, stone2) = found_dividers.entry(stone).or_insert({
                    let midpoint = stone.to_string().len() / 2;
                    let divider = 10_u64.pow(midpoint as u32);
                    let (stone1, stone2) = (stone / divider as u64, stone % divider as u64);
                    (stone1, stone2)
                });

                *new_stones.entry(*stone1).or_insert(0) += count;
                *new_stones.entry(*stone2).or_insert(0) += count;
            }
            _ => {
                *new_stones
                    .entry(*found_multipliers.entry(stone).or_insert(stone * 2024))
                    .or_insert(0) += count;
            }
        }
    }
    new_stones
}
