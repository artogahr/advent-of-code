use std::fs::read_to_string;

fn main() {
    let banks: Vec<Vec<u32>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut total = 0;

    for bank in banks {
        let mut max_index = 0;
        for i in 1..bank.len() - 1 {
            if bank[i] > bank[max_index] {
                max_index = i;
            }
        }
        let mut second_max_index = max_index + 1;
        for j in max_index + 2..bank.len() {
            if bank[j] > bank[second_max_index] {
                second_max_index = j;
            }
        }
        total += 10 * bank[max_index] + bank[second_max_index];
    }

    dbg!(total);
}
