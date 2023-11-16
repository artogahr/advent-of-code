use std::io;

fn main() {
    let mut numbers: Vec<i32> = vec![];
    let mut current_iter = 1;
    let mut previous_iter = 0;
    let mut count = 0;
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        if input_text.is_empty() {
            break;
        }

        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => {
                numbers.push(i);
            }
            Err(..) => {
                break;
            }
        };
    }
    loop {
        let current_iter_sum =
            numbers[current_iter] + numbers[current_iter + 1] + numbers[current_iter + 2];
        let prev_item_sum =
            numbers[previous_iter] + numbers[previous_iter + 1] + numbers[previous_iter + 2];
        if current_iter_sum > prev_item_sum {
            count += 1;
        }
        if current_iter + 3 == numbers.len() {
            break;
        }
        current_iter += 1;
        previous_iter += 1;
    }
    println!("{}", count);
}
