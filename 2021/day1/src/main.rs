use std::io;

fn main() {
    let mut numbers: Vec<i32> = vec![];
    let mut currentIter = 1;
    let mut previousIter = 0;
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
        let currentIterSum =
            numbers[currentIter] + numbers[currentIter + 1] + numbers[currentIter + 2];
        let prevItemSum =
            numbers[previousIter] + numbers[previousIter + 1] + numbers[previousIter + 2];
        if currentIterSum > prevItemSum {
            count += 1;
        }
        if currentIter + 3 == numbers.len() {
            break;
        }
        currentIter += 1;
        previousIter += 1;
    }
    println!("{}", count);
}
