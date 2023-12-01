/// The input function can return any type that implements Clone
fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

/// The part functions must take the input as an argument and return
/// anything implementing Display
fn part1(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let num_in_lines: Vec<u32> = line.chars()
            .filter(|char| char.is_digit(10))
            .map(|c| c.to_digit(10).unwrap()).collect();
        println!("{:?} {} {}", num_in_lines, num_in_lines.first().unwrap(), num_in_lines.last().unwrap());
        sum += 10 * num_in_lines.first().unwrap();
        sum += num_in_lines.last().unwrap();
    }
    sum 
}

fn part2(input: Vec<String>) -> i64 {
    todo!()
}

fn main() {
    let solution = advent::new(parse_input)
        .part(part1)
        .part(part2)
        .build();
    solution.cli()
}
