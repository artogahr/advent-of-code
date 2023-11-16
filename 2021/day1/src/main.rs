use std::io;

fn main() {
    let mut currentNum = 0;
    let mut previousNum = 0;
    let mut counter = 0;
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        if input_text.is_empty() {
            break;
        }
    
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => currentNum = i,
            Err(..) => {
                break;
            },
        };
        if currentNum > previousNum {
            counter += 1;
        }
        previousNum = currentNum;
    }
    print!("{}", counter-1);
}