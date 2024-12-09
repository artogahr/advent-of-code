use std::{fs, iter, u32};

use itertools::Itertools;

fn main() {
    let disk_map: Vec<usize> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|ch| u32::from(ch as u32 - 48) as usize)
        .collect();
    part1(&disk_map);
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

fn print_map(expanded_map: &Vec<Option<usize>>) {
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
