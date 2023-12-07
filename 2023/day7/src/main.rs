use std::vec;

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_hands(lines: Vec<String>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let (hands_str, bid) = line.split_once(' ').unwrap();
        hands.push(Hand {
            cards: hands_str.chars().collect(),
            bid: bid.parse::<u32>().unwrap(),
        });
    }
    sort_hands(hands)
}

fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl Hand {
    fn bigger(&self, other: &Self) -> bool {
        false
    }
}

fn part1(input: Vec<String>) -> u32 {
    let hands = parse_hands(input);
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * hand.bid;
    }
    sum
}

fn part2(input: Vec<String>) -> u32 {
    todo!();
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn test_bigger() {
    let hands = parse_hands(
        "32T3K 765\nT55J5 684"
            .to_string()
            .lines()
            .map(str::parse)
            .map(Result::unwrap)
            .collect(),
    );
    assert!(
        hands
            .iter()
            .nth(0)
            .unwrap()
            .bigger(hands.iter().nth(1).unwrap())
            == false
    );
}
