use std::collections::HashMap;



fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part1(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let colon_index = line.find(':').unwrap();
        let line = line[colon_index + 1..].trim();
        let pipe_index = line.find('|').unwrap();
        let (left_side, right_side) = line.split_at(pipe_index);
        let win_numbers: Vec<u32> = left_side
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Couldn't parse u32"))
            .collect();
        let our_numbers: Vec<u32> = right_side[1..]
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Couldn't parse u32"))
            .collect();
        let mut card_score: u32 = 0;
        for number in our_numbers {
            if win_numbers.contains(&number) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        sum += card_score;
    }
    sum
}

fn part2(input: Vec<String>) -> u32 {
    let mut card_counts: HashMap<usize, u32> = HashMap::new();
    card_counts.insert(0, 1);
    for (card_i, line) in input.iter().enumerate() {
        //dbg!(&card_counts);
        let remaining_tries = &mut card_counts.entry(card_i).or_insert(1).clone();
        while remaining_tries > &mut 0 {
            //println!("Checking for card {}", card_i+1);
            let colon_index = line.find(':').unwrap();
            let card = line[colon_index + 1..].trim();
            let pipe_index = card.find('|').unwrap();
            let (left_side, right_side) = card.split_at(pipe_index);

            let win_numbers: Vec<u32> = left_side
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("Couldn't parse u32"))
                .collect();
            let our_numbers: Vec<u32> = right_side[1..]
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("Couldn't parse u32"))
                .collect();

            let mut win_counter: usize = 0;
            for number in our_numbers {
                if win_numbers.contains(&number) {
                    win_counter += 1;
                }
            }
            for j in 1..=win_counter {
                *card_counts.entry(card_i + j).or_insert(1) += 1;
            }

            *remaining_tries -= 1;
        }
    }
    let mut sum: u32 = 0;
    for (_, card) in card_counts {
        sum += card;
    }
    sum
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
