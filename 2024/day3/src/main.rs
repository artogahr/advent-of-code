use std::fs::read_to_string;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let file = read_to_string("input.txt").unwrap();
    let mut sum = 0;
    'main: for (i, slice) in file.as_bytes().windows(4).enumerate() {
        if slice == [109, 117, 108, 40] {
            //        m,   u,   l,  (
            // Look at the part until ',', try to determine if it's a valid number
            for (j, char) in file.as_bytes().iter().skip(i + 4).enumerate() {
                if *char == ',' as u8 {
                    let new_slice: Vec<&u8> = file.as_bytes().iter().skip(i + 4).take(j).collect();
                    let first_number = String::from_utf8(new_slice.iter().map(|u| **u).collect())
                        .unwrap()
                        .parse::<u32>();
                    match first_number {
                        Ok(first_number) => {
                            // Look at the part until ',', try to determine if it's a valid number
                            for (k, char) in file.as_bytes().iter().skip(i + 4 + j + 1).enumerate()
                            {
                                if *char == ')' as u8 {
                                    let new_slice: Vec<&u8> = file
                                        .as_bytes()
                                        .iter()
                                        .skip(i + 4 + j + 1)
                                        .take(k)
                                        .collect();
                                    let second_number =
                                        String::from_utf8(new_slice.iter().map(|u| **u).collect())
                                            .unwrap()
                                            .parse::<u32>();
                                    match second_number {
                                        Ok(second_number) => {
                                            sum += first_number * second_number;
                                        }
                                        Err(_) => {
                                            continue 'main;
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            continue 'main;
                        }
                    }
                    break;
                }
            }
            //let first_number = file.chars().skip(i+4).take(n)
        }
    }
    sum
}

fn part2() -> u32 {
    let file = read_to_string("input.txt").unwrap();
    let mut sum = 0;
    let mut mul_enabled: bool = true;
    'main: for (i, slice) in file.as_bytes().windows(7).enumerate() {
        if slice[0..4] == [109, 117, 108, 40] && mul_enabled {
            //              m,   u,   l,  (
            // Look at the part until ',', try to determine if it's a valid number
            for (j, char) in file.as_bytes().iter().skip(i + 4).enumerate() {
                if *char == ',' as u8 {
                    let new_slice: Vec<&u8> = file.as_bytes().iter().skip(i + 4).take(j).collect();
                    let first_number = String::from_utf8(new_slice.iter().map(|u| **u).collect())
                        .unwrap()
                        .parse::<u32>();
                    match first_number {
                        Ok(first_number) => {
                            // Look at the part until ',', try to determine if it's a valid number
                            for (k, char) in file.as_bytes().iter().skip(i + 4 + j + 1).enumerate()
                            {
                                if *char == ')' as u8 {
                                    let new_slice: Vec<&u8> = file
                                        .as_bytes()
                                        .iter()
                                        .skip(i + 4 + j + 1)
                                        .take(k)
                                        .collect();
                                    let second_number =
                                        String::from_utf8(new_slice.iter().map(|u| **u).collect())
                                            .unwrap()
                                            .parse::<u32>();
                                    match second_number {
                                        Ok(second_number) => {
                                            sum += first_number * second_number;
                                        }
                                        Err(_) => {
                                            continue 'main;
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            continue 'main;
                        }
                    }
                    break;
                }
            }
            //let first_number = file.chars().skip(i+4).take(n)
        } else if slice[0..4] == [100, 111, 40, 41] {
            //                      d    o    (   )
            mul_enabled = true;
        } else if slice == [100, 111, 110, 39, 116, 40, 41] {
            //               d    o    n   '   t    (   )
            mul_enabled = false;
        }
    }
    sum
}
