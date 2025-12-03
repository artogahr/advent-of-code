use std::fs::read_to_string;

fn main() {
    let banks: Vec<Vec<u32>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut total: u64 = 0;

    for bank in banks {
        let mut current_max_index = 0;

        // println!("Checking bank: {:?}", bank);

        for digit in (0..12).rev() {
            // print!("Checking digit no {digit}: ");
            for i in current_max_index + 1..bank.len() - digit {
                if bank[i] > bank[current_max_index] {
                    current_max_index = i;
                }
            }
            // println!(
            //     "found at index {current_max_index}: {}",
            //     bank[current_max_index]
            // );
            total += 10_u64.pow(digit as u32) * bank[current_max_index] as u64;
            current_max_index += 1;
        }
    }

    dbg!(total);
}
