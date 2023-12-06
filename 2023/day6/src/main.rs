fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn part1(input: Vec<String>) -> u32 {
    let mut races: Vec<Race> = Vec::new();
    let times: Vec<u32> = input
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|n| n.parse().ok())
        .collect();
    let distances: Vec<u32> = input
        .iter()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|n| n.parse().ok())
        .collect();

    for (i, time) in times.iter().enumerate() {
        races.push(Race {
            time: *time,
            distance: distances[i],
        });
    }

    races
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|&hold_time| (race.time - hold_time) * hold_time > race.distance)
                .count() as u32
        })
        .product()
}

fn part2(input: Vec<String>) -> u32 {
    let time: String = input.first().unwrap().split_whitespace().skip(1).collect();
    let distance: String = input
        .iter()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect();

    let time: u64 = time.parse().unwrap();
    let distance: u64 = distance.parse().unwrap();

    let mut result: u32 = 0;
    for hold_time in 0..time {
        if (time - hold_time) * hold_time > distance {
            result += 1;
        }
    }
    result
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
