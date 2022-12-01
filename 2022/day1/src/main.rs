use std::io;
use std::io::Write;

fn main() {
    let mut max_number: i32 = 0;
    let mut second_max: i32 = 0;
    let mut third_max: i32 = 0;
    let mut current_number: i32 = 0;
    let mut is_end_of_input: bool = false;
    loop {
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
        }
        else {
            if is_end_of_input {
                break;
            } 
                        if current_number > third_max {
                if current_number > second_max {
                    if current_number > max_number {
                        third_max = second_max;
                        second_max = max_number;
                        max_number = current_number;
                        //println!("max number became {max_number}");
                        //println!("so second number became {second_max}");
                        //println!("so third number became {third_max}");
                    }
                    else {
                        third_max = second_max;
                        second_max = current_number;
                        // println!("second number became {second_max}");
                        // println!("so third number became {third_max}");
                    }
                }
                else {
                    third_max = current_number;
                        // println!("third number became {third_max}");
                }
            }

            current_number = 0;
            is_end_of_input = true;
        }
    }
    let maxi = max_number + second_max + third_max;
    println!("{maxi}");
}
