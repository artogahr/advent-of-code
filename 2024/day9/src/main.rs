use std::{fs, iter, u32};

fn main() {
    let disk_map: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|ch| u32::from(ch as u32 - 48) as usize)
        .collect();
    part1(&disk_map);
    part2(&disk_map);
}

fn part1(disk_map: &Vec<usize>) {
    let mut expanded_map: Vec<Option<usize>> = Vec::new();
    let mut file_id = 0;
    for (file_size, empty_space) in disk_map.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])) {
        // print!("Adding file {file_id} with size {file_size} \t");
        expanded_map.extend(iter::repeat(Some(file_id)).take(file_size));
        // println!("Adding empty space with size {empty_space}");
        expanded_map.extend(iter::repeat(None).take(empty_space));
        file_id += 1;
    }
    if disk_map.len() % 2 != 0 {
        // println!(
        //     "Adding file {file_id} with size {0} \t",
        //     disk_map.last().unwrap()
        // );
        expanded_map.extend(iter::repeat(Some(file_id)).take(*disk_map.last().unwrap()));
    }

    while expanded_map.contains(&None) {
        for mut i in 0.. {
            trim_memory(&mut expanded_map);
            if expanded_map[i].is_none() {
                let len = expanded_map.len();
                expanded_map.swap(i, len - 1);
                expanded_map.pop();
                i -= 1;
                //print_map(&expanded_map);
            }
            if i == expanded_map.len() - 1 {
                break;
            }
        }
    }

    let sum: usize = expanded_map
        .iter()
        .enumerate()
        .map(|(i, block)| i * block.unwrap())
        .sum();

    println!("Part 1: After compacting, memory checksum is: {sum}");
}

fn print_expanded_map(expanded_map: &Vec<Option<usize>>) {
    for block in expanded_map {
        match block {
            Some(number) => print!("{number}"),
            None => print!("."),
        }
    }
    println!();
}

fn trim_memory(expanded_map: &mut Vec<Option<usize>>) {
    while expanded_map.last().unwrap().is_none() {
        expanded_map.pop();
    }
}

// 0123456789
// 2 3 1 3 2
//  3 3 3 1
//  len 9
fn part2(disk_map: &Vec<usize>) {
    let mut expanded_map: Vec<Option<usize>> = Vec::new();
    let mut file_id = 0;

    for (file_size, empty_space) in disk_map.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])) {
        expanded_map.extend(iter::repeat(Some(file_id)).take(file_size));
        expanded_map.extend(iter::repeat(None).take(empty_space));
        file_id += 1;
    }
    if disk_map.len() % 2 != 0 {
        expanded_map.extend(iter::repeat(Some(file_id)).take(*disk_map.last().unwrap()));
    }

    for current_file_id in (0..file_id + 1).rev() {
        let mut file_start = None;
        let mut file_size = 0;

        for (i, block) in expanded_map.iter().enumerate() {
            if *block == Some(current_file_id) {
                file_start.get_or_insert(i);
                file_size += 1;
            } else if file_start.is_some() {
                break;
            }
        }

        if let Some(file_start_index) = file_start {
            let mut free_start = None;
            let mut free_size = 0;

            for i in 0..file_start_index {
                if expanded_map[i] == None {
                    free_start.get_or_insert(i);
                    free_size += 1;

                    if free_size == file_size {
                        let free_start_index = free_start.unwrap();
                        for j in 0..file_size {
                            expanded_map[free_start_index + j] = Some(current_file_id);
                            expanded_map[file_start_index + j] = None;
                        }
                        break;
                    }
                } else {
                    free_start = None;
                    free_size = 0;
                }
            }
        }

        //print_expanded_map(&expanded_map);
    }

    let checksum: usize = expanded_map
        .iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|id| i * id))
        .sum();

    println!("Part 2: After compacting, memory checksum is: {checksum}");
}
