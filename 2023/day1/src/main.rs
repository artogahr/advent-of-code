fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

/// anything implementing Display
fn part1(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let num_in_lines: Vec<u32> = line
            .chars()
            .filter(|char| char.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        sum += 10 * num_in_lines.first().unwrap();
        sum += num_in_lines.last().unwrap();
    }
    sum
}

fn part2(input: Vec<String>) -> u32 {
    println!("Running part 2 ----------------------------");
    let digits: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];
    let mut sum: u32 = 0;
    for line in input {
        //println!("{line}");
        let mut first_str_pos: i32 = 999;
        let mut last_str_pos: i32 = -1;
        let mut first_num_pos: i32 = 999;
        let mut last_num_pos: i32 = -1;
        let nums_in_lines: Vec<u32> = line
            .chars()
            .filter(|char| char.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        for digit in digits.clone() {
            let mut cur_line = line.clone();
            let mut removed_count = 0;
            //println!("\nlooking for {digit}");
            loop {
                match cur_line.find(&digit) {
                    Some(pos) => {
                        if pos as i32 > last_str_pos {
                            last_str_pos = pos as i32 + removed_count as i32;
                        }
                        if (pos as i32) < first_str_pos {
                            first_str_pos = pos as i32;
                        }
                        //println!("while removing: fs: {first_str_pos}, ls: {last_str_pos}");
                        //println!("removing from line {cur_line}");
                        cur_line.replace_range(pos+1..pos + digit.len()-1, "");
                        //println!("after removing: {cur_line}");
                        removed_count += 1;
                    }
                    None => {break;}
                }
                //println!("fs {first_str_pos} ls {last_str_pos}");
            }
            match nums_in_lines.first() {
                Some(num) => {
                    first_num_pos = line.find(&num.to_string()).unwrap() as i32;
                }
                None => {
                    first_num_pos = 999;
                }
            }
            match nums_in_lines.last() {
                Some(num) => {
                    last_num_pos = line.rfind(&num.to_string()).unwrap() as i32;
                }
                None => {
                    last_num_pos = -1;
                }
            }
        }
        println!(
            "for {line} fs: {first_str_pos} ls {last_str_pos} fn {first_num_pos} ln {last_num_pos}"
        );
        match (first_str_pos < first_num_pos, last_str_pos > last_num_pos) {
            (true, true) => {
                let var_name = 10 * get_num_from_pos(&line, first_str_pos)
                    + get_last_num_from_pos(&line, last_str_pos);
                println!("found num {var_name}");
                sum += var_name as u32;
            }
            (false, true) => {
                let var_name = 10 * nums_in_lines.first().unwrap()
                    + get_last_num_from_pos(&line, last_str_pos) as u32;
                println!("found num {var_name}");
                sum += var_name as u32;
            }
            (true, false) => {
                let var_name = 10 * get_num_from_pos(&line, first_str_pos) as u32
                    + nums_in_lines.last().unwrap();
                println!("found num {var_name}");
                sum += var_name as u32;
            }
            (false, false) => {
                let var_name =
                    10 * nums_in_lines.first().unwrap_or(&0) + nums_in_lines.last().unwrap_or(&0);
                println!("found num {var_name}");
                sum += var_name as u32;
            }
        }
    println!("");
    }
    sum
}

fn get_num_from_pos(line: &String, str_pos: i32) -> i32 {
    let digits: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];
    for (i, digit) in digits.iter().enumerate() {
        //println!("finding {digit} in {line}");
        if line.find(digit).unwrap_or(999) == str_pos as usize {
            //line.replace_range((str_pos as usize..str_pos as usize +digit.len()), "");
            return i as i32 + 1;
        } else {
            continue;
        }
    }
    0
}
fn get_last_num_from_pos(line: &String, str_pos: i32) -> i32 {
    let digits: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];
    for (i, digit) in digits.iter().enumerate() {
        let mut pos = 0;
        while let Some(found_pos) = &line[pos..].find(digit) {
            pos += found_pos + digit.len();
            if pos - digit.len() as usize == str_pos as usize {
                return i as i32 + 1;
            }
        }
    }
    0
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
