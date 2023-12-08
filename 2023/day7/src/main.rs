use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = self.hand_type().cmp(&other.hand_type());
        if type_ordering != Ordering::Equal {
            return type_ordering.reverse();
        }
        //dbg!(self, other);
        //println!("above two are same, checking based on cards");
        for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
            let self_value = self_card.to_digit(10).unwrap_or_else(|| match self_card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => self_card.to_digit(10).unwrap(),
            });

            let other_value = other_card.to_digit(10).unwrap_or_else(|| match other_card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => other_card.to_digit(10).unwrap(),
            });

            let value_ordering = self_value.cmp(&other_value);
            if value_ordering != Ordering::Equal {
                return value_ordering; // Reverse the ordering if values are not equal
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts = [0; 13];
        let mut j_count: i32 = 0;
        for &card in &self.cards {
            match card {
                'A' => counts[0] += 1,
                'K' => counts[1] += 1,
                'Q' => counts[2] += 1,
                'J' => j_count += 1,
                'T' => counts[4] += 1,
                '9' => counts[5] += 1,
                '8' => counts[6] += 1,
                '7' => counts[7] += 1,
                '6' => counts[8] += 1,
                '5' => counts[9] += 1,
                '4' => counts[10] += 1,
                '3' => counts[11] += 1,
                '2' => counts[12] += 1,
                _ => unreachable!(),
            }
        }

        let num_unique_values = counts.iter().filter(|&&count| count > 0).count();
        let max_count = *counts.iter().max().unwrap() + j_count;
        if j_count == 5 {
            return HandType::FiveOfAKind;
        }
        match (num_unique_values, max_count) {
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::to_string)
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
    hands
}

fn calculate_winnings(hands: &[Hand]) -> u32 {
    let mut total_winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank_sum = (i as u32 + 1) * hand.bid;
        total_winnings += rank_sum;
        //dbg!(&hand, &hand.hand_type(), i + 1, rank_sum, total_winnings);
    }
    total_winnings
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| a.cmp(b));
}

fn part1(input: Vec<String>) -> u32 {
    let mut hands = parse_hands(input);
    sort_hands(&mut hands);
    let total_winnings = calculate_winnings(&hands);
    total_winnings
}

fn part2(input: Vec<String>) -> u32 {
    let mut hands = parse_hands(input);
    sort_hands(&mut hands);
    let total_winnings = calculate_winnings(&hands);
    total_winnings
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    println!("See git history for correct solution to part 1");
    solution.cli()
}

#[test]
fn test_hand_type() {
    // Test case for Five of a Kind
    let hand = Hand {
        cards: vec!['A', 'A', 'A', 'A', 'A'],
        bid: 123,
    };
    assert_eq!(hand.hand_type(), HandType::FiveOfAKind);

    // Test case for Four of a Kind
    let hand = Hand {
        cards: vec!['A', 'A', 'A', '8', 'A'],
        bid: 456,
    };
    assert_eq!(hand.hand_type(), HandType::FourOfAKind);

    // Test case for Full House
    let hand = Hand {
        cards: vec!['2', '3', '3', '3', '2'],
        bid: 789,
    };
    assert_eq!(hand.hand_type(), HandType::FullHouse);

    // Add more test cases for other hand types

    // Test case for High Card
    let hand = Hand {
        cards: vec!['2', '3', '4', '5', '6'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::HighCard);

    let hand = Hand {
        cards: vec!['3', '2', 'T', '3', 'K'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::OnePair);
    // Test case for J
    let hand = Hand {
        cards: vec!['T', '5', '5', 'J', '5'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::FourOfAKind);
    // Test case for J 2
    let hand = Hand {
        cards: vec!['J', '5', '5', 'J', '5'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::FiveOfAKind);

    let hand = Hand {
        cards: vec!['3', '2', 'J', 'Q', 'K'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::OnePair);
    let hand = Hand {
        cards: vec!['2', 'J', 'J', 'J', 'J'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    let hand = Hand {
        cards: vec!['J', 'J', 'J', 'J', 'J'],
        bid: 987,
    };
    assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
}

#[test]
fn test_compare_same_type_same_first_card_higher_second_card() {
    let hand1 = Hand {
        cards: vec!['T', '5', '5', 'J', '5'],
        bid: 123,
    };
    let hand2 = Hand {
        cards: vec!['Q', 'Q', 'Q', 'J', 'A'],
        bid: 456,
    };
    assert_eq!(hand1.cmp(&hand2), Ordering::Less);
    assert_eq!(hand2.cmp(&hand1), Ordering::Greater);
}
