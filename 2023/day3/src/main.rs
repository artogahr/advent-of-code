
use std::vec;

#[derive(Debug)]
struct Number {
    start_index: usize,
    end_index: usize,
    line_index: usize,
    value: u32,
}

fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part1(input: Vec<String>) -> u32 {
    //empty chars 2d array, 1 extra wall in each direction to avoid over/underflows
    let mut chars: Vec<Vec<char>> = vec![vec!['.'; input[0].len() + 2]; input.len() + 2];
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            chars[i + 1][j + 1] = ch;
        }
    }
    let mut numbers: Vec<Number> = vec![];
    let mut is_reading_num = false;
    let mut current_num: u32 = 0;
    for (i, line) in chars.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            match ch {
                ('0'..='9') => {
                    if is_reading_num {
                        current_num *= 10;
                        current_num += ch.to_digit(10).unwrap();
                    } else {
                        current_num = ch.to_digit(10).unwrap();
                    }
                    is_reading_num = true;
                }
                _ => {
                    if is_reading_num {
                        is_reading_num = false;
                        numbers.push(Number {
                            start_index: j - current_num.to_string().len(),
                            end_index: j - 1,
                            line_index: i,
                            value: current_num,
                        });
                        //dbg!(&numbers);
                    }
                }
            }
        }
    }
    //dbg!(&numbers);
    let mut sum: u32 = 0;
    'number_looop: for number in &numbers {
        for line_i in number.line_index - 1..=number.line_index + 1 {
            for char_i in number.start_index - 1..=number.end_index + 1 {
                //dbg!(number.value, line_i, char_i, chars[line_i][char_i]);
                if chars[line_i][char_i] != '.' && !chars[line_i][char_i].is_ascii_digit() {
                    //println!(  "number {} has char {} in surrounding", number.value, chars[line_i][char_i]);
                    sum += number.value;
                    continue 'number_looop;
                }
            }
        }
        //println!("number {} has nothing in surrounding", number.value);
    }
    dbg!(&numbers.len());
    sum
}

fn part2(input: Vec<String>) -> u32 {
    //empty chars 2d array, 1 extra wall in each direction to avoid over/underflows
    let mut chars: Vec<Vec<char>> = vec![vec!['.'; input[0].len() + 2]; input.len() + 2];
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            chars[i + 1][j + 1] = ch;
        }
    }
    let mut numbers: Vec<Number> = vec![];
    let mut is_reading_num = false;
    let mut current_num: u32 = 0;
    for (i, line) in chars.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            match ch {
                ('0'..='9') => {
                    if is_reading_num {
                        current_num *= 10;
                        current_num += ch.to_digit(10).unwrap();
                    } else {
                        current_num = ch.to_digit(10).unwrap();
                    }
                    is_reading_num = true;
                }
                _ => {
                    if is_reading_num {
                        is_reading_num = false;
                        numbers.push(Number {
                            start_index: j - current_num.to_string().len(),
                            end_index: j - 1,
                            line_index: i,
                            value: current_num,
                        });
                        //dbg!(&numbers);
                    }
                }
            }
        }
    }
    let mut sum =0;
    for (i, line) in chars.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            let _gear_ratio = 0;
            let mut first_num = 0;
            let _second_nu = 0;
            match ch {
                '*' => {
                    for number in &numbers {
                        if number.line_index >= i-1 && number.line_index <= i+1 && number.end_index >= j-1 && number.start_index <= j+1 {
                            if first_num == 0{
                                first_num = number.value;
                            }
                            else {
                                sum += first_num * number.value;
                            }
                        }
                    }
                }
                _ => {
                }
            }
        }
    }
    sum
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
