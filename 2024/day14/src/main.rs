use core::panic;
use std::{fs, thread::sleep, time::Duration, usize};

use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    position: (usize, usize),
    velocity: (i32, i32),
}

impl Robot {
    fn move_one_second(&mut self) {
        self.position.0 = ((self.position.0 as i32 + self.velocity.0 + 101) % 101).abs() as usize;
        self.position.1 = ((self.position.1 as i32 + self.velocity.1 + 103) % 103).abs() as usize;
    }
}

fn main() {
    println!("Part 1: {0}", part1(read_input()));
    println!("Part 2: {0}", part2(read_input()));
}

fn part1(robots: Vec<Robot>) -> usize {
    let mut robots = robots.clone();
    // print_map(&robots, 101, 103);
    for _ in 0..1000 {
        for robot in robots.iter_mut() {
            robot.move_one_second();
        }
        //print_map(&robots, 101, 103);
        //sleep(Duration::from_millis(300));
        //print!("{}[2J", 27 as char);
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    //print_map(&robots, 101, 103);

    //dbg!(&robots);

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robots {
        match robot.position {
            (x, y) if x < 101 / 2 && y < 103 / 2 => q1 += 1,
            (x, y) if x >= 101 / 2 + 1 && y < 103 / 2 => q2 += 1,
            (x, y) if x < 101 / 2 && y >= 103 / 2 + 1 => q3 += 1,
            (x, y) if x >= 101 / 2 + 1 && y >= 103 / 2 + 1 => q4 += 1,
            _ => {}
        };
    }
    q1 * q2 * q3 * q4
}

fn part2(original_robots: Vec<Robot>) -> usize {
    let mut robots = original_robots.clone();
    let mut lowest_score_index = 0;
    let mut lowest_danger_score = usize::MAX;
    for i in 1..103 * 107 {
        for robot in robots.iter_mut() {
            robot.move_one_second();
        }
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for robot in &robots {
            match robot.position {
                (x, y) if x < 101 / 2 && y < 103 / 2 => q1 += 1,
                (x, y) if x >= 101 / 2 + 1 && y < 103 / 2 => q2 += 1,
                (x, y) if x < 101 / 2 && y >= 103 / 2 + 1 => q3 += 1,
                (x, y) if x >= 101 / 2 + 1 && y >= 103 / 2 + 1 => q4 += 1,
                _ => {}
            };
        }
        let danger_score = q1 * q2 * q3 * q4;
        if danger_score < lowest_danger_score {
            lowest_danger_score = danger_score;
            lowest_score_index = i;
        }
    }
    lowest_score_index
}
fn _print_map(robots: &Vec<Robot>, x: usize, y: usize) {
    let mut map: Vec<Vec<usize>> = vec![vec![0; x]; y];

    println!("-----------------");
    for robot in robots {
        map[robot.position.1][robot.position.0] += 1;
    }
    for line in map {
        for num in line {
            if num == 0 {
                print!(".");
            } else {
                print!("{num}");
            }
        }
        println!();
    }
    sleep(Duration::from_millis(300));
    print!("{}[2j", 27 as char);
    print!("{esc}[2j{esc}[1;1h", esc = 27 as char);
}

fn read_input() -> Vec<Robot> {
    let file = fs::read_to_string("input.txt").unwrap();
    let pattern = Regex::new("p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    file.lines()
        .map(|line| {
            let Some(caps) = pattern.captures(line) else {
                panic!("Couldn't parse all lines")
            };
            Robot {
                position: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                velocity: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            }
        })
        .collect()
}
