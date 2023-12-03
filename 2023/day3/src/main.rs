use std::collections::HashMap;

struct Number {
    start_index: u32,
    end_index: u32, 
    value: u32,
}

fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part1(input: Vec<String>) -> i64 {
    let numbers: Vec<Number> = vec![];
    for line in input { 
        let line = line.replace(".", " ");
        dbg!(line);
    }
    0
}

fn part2(input: Vec<String>) -> i64 {todo!();}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
