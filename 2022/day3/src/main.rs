use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    println!("Tip: remember to press newline a couple of times after entering input");

    let mut lines = io::stdin().lock().lines();
    let mut sum: u32 = 0;

    'lineread: while let Some(line1) = lines.next() {
        let line1 = line1.unwrap();
        let line2 = lines.next().unwrap()?;
        let line3 = lines.next().unwrap()?;

        let line1_chars: Vec<char> = line1.trim().chars().collect();
        let line2_chars: Vec<char> = line2.chars().collect();
        let line3_chars: Vec<char> = line3.chars().collect();

        if line1_chars.is_empty() || line2_chars.is_empty() || line3_chars.is_empty() {
            break 'lineread;
        }

        for char in line1_chars {
            if line2_chars.contains(&char) && line3_chars.contains(&char) {
                    if (char as u32) < 97 {
                        let value = (char as u32) - 38;
                        sum += value;
                        // println!("found char {0}, adding {1}", char, value);
                        continue 'lineread;
                    }
                    else {
                        let value = (char as u32) - 96;
                        sum += value;
                        // println!("found char {0}, adding {1}", char, value);
                        continue 'lineread;
                    }
            }
        }
    }
    println!("Sum: {0}", sum);
    Ok(())
}
