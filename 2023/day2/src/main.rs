use std::collections::HashMap;

fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part1(input: Vec<String>) -> i64 {
    let target_cubes: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    // Calculate the sum of IDs for possible games
    // Print the result
    input
        .iter()
        .filter(|&game| is_possible_game(&target_cubes, game))
        .flat_map(|game| {
            game.split_whitespace()
                .nth(1)
                .and_then(|s| s.trim_end_matches(':').parse::<i64>().ok())
        })
        .sum()
}

fn is_possible_game(cubes: &HashMap<&str, u32>, game: &str) -> bool {
    //dbg!(game);
    let colon_index = game.find(':').unwrap();
    let plays = &game[colon_index + 1..].trim();
    for play in plays.split(';') {
        for pull in play.split(',') {
            let (number, color) = pull.trim().split_at(pull.trim().find(' ').unwrap());
            if cubes.get(color.trim()).unwrap_or(&0) < &number.parse::<u32>().unwrap() {
                return false;
            }
        }
    }
    true
}

fn part2(input: Vec<String>) -> i64 {
    let mut cubes: HashMap<String, u32> = [
        ("red".to_string(), 0),
        ("green".to_string(), 0),
        ("blue".to_string(), 0),
    ]
    .iter()
    .cloned()
    .collect();
    let mut sum: i64 = 0;
    for game in input {
        cubes.insert("red".to_string(), 0);
        cubes.insert("blue".to_string(), 0);
        cubes.insert("green".to_string(), 0);
        let colon_index = game.find(':').unwrap();
        let plays = game[colon_index + 1..].trim().to_string(); // Clone the substring

        for play in plays.split(';') {
            for pull in play.split(',') {
                let (number, color) = pull.trim().split_at(pull.trim().find(' ').unwrap());
                let number = number.parse::<u32>().unwrap();
                if number > *cubes.get(&color.trim().to_string()).unwrap_or(&0) {
                    cubes.insert(color.trim().to_string(), number);
                }
                //dbg!(&cubes);
            }
        }
        sum += *cubes.get("red").unwrap() as i64
            * *cubes.get("green").unwrap() as i64
            * *cubes.get("blue").unwrap() as i64;
        //dbg!(sum);
    }
    sum
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
