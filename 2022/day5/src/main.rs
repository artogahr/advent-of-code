use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let file_handle = File::open("input.txt").expect("Couldn't open file");
    let file = BufReader::new(file_handle);

    let mut stack_index;
    let mut has_encountered_space = false;
    let mut space_continue_index = 0;
    let mut array: Vec<Vec<char>> = Vec::new();

    'first_loop: for line in file.lines() {
        stack_index = 0;
        for ch in line.expect("Unable to read line").chars() {
            match ch {
                '[' | ']' => {
                    has_encountered_space = false;
                    continue;
                },
                ' ' => {
                    if !has_encountered_space {
                        has_encountered_space = true;
                        continue;
                    }
                    else if space_continue_index == 3 {
                        if array.len() <= stack_index {
                            // println!("{0} {1}", array.len(), stack_index);
                            array.push(Vec::new());
                        }
                        stack_index += 1;
                        has_encountered_space = true;
                        space_continue_index = 0;
                        continue;
                    }
                    else {
                        space_continue_index += 1;
                        continue;
                    }
                },
                'A'..='Z' => {
                    if array.len() <= stack_index {
                        // println!("{0} {1}", array.len(), stack_index);
                        array.push(Vec::new());
                    } 
                    array[stack_index].push(ch);
                    stack_index += 1;
                },
                '1' => {break 'first_loop},
                _ => continue,
            }
        }
    }
    let file_handle = File::open("input.txt").expect("Couldn't open file");
    let file = BufReader::new(file_handle);
    'second_loop: for line in file.lines() {
        let words: Vec<String> = line.expect("Couldn't read any more lines")
            .split(' ').map(|s| s.to_string()).collect();
        if words[0] != "move" {
            continue;
        }
        else {
            let move_count: i32 = words[1].parse().unwrap();
            let from_stack: i32 = words[3].parse().unwrap();
            let to_stack: i32   = words[5].parse().unwrap();
        }
    }
    // array.iter().for_each(|it| {
    //     println!("{:#?}", it);
    // });
}
