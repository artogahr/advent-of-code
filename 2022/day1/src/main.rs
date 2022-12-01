use std::io;
use std::io::Write;

fn main() {
    let mut max_number: i32 = 0;
    let mut current_number: i32 = 0;
    let mut is_end_of_input: bool = false;
    loop {
        print!("prompt> ");
        io::stdout().flush().expect("couldn't flush");
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("couldn't read");
        let mut str = str.trim();
        if str.is_empty() {
            str = " ";
        }
        let input_is_numeric = str.chars().all(char::is_numeric);
        if input_is_numeric {
            is_end_of_input = false;
            current_number += str.to_string().parse::<i32>().unwrap();
            if current_number > max_number {
                max_number = current_number;
            }
        }
        else {
            if is_end_of_input {
                break;
            } 
            current_number = 0;
            is_end_of_input = true;
        }
    }
    println!("{max_number}");
}
