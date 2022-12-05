use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let file = BufReader::new(File::open("input.txt").expect("Unable to open file"));

    let mut stack_index;
    let mut has_encountered_space = false;
    let mut space_continue_index = 0;
    let mut array: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
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
                _ => continue,
            }
        }
    }
    array.iter().for_each(|it| {
        println!("{:#?}", it);
    })
}
