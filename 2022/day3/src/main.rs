use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();
    let mut sum: u32 = 0;

    'lineread: while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.is_empty() {
            break;
        }

        let char_vec: Vec<char> = last_input.chars().collect();
        if char_vec.is_empty() {
            break;
        }
        let (first, second) = char_vec.split_at(char_vec.len()/2);

        for char in first {
            if second.contains(char) {
                if (*char as u32) < 97 {
                    let value = (*char as u32) - 38;
                    sum += value;
                    println!("found char {0}, adding {1}", char, value);
                    continue 'lineread;
                }
                else {
                    let value = (*char as u32) - 96;
                    sum += value;
                    println!("found char {0}, adding {1}", char, value);
                    continue 'lineread;
                }

}
        }

        // add a new line once user_input starts storing user input
    }

    println!("Sum: {0}", sum);

    // the lock is released after it goes out of scope
    Ok(())
}
