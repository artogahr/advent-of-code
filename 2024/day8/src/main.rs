use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("input.txt").unwrap());

    let antenna_map: Vec<Vec<char>> = f.lines().map(|l| l.unwrap().chars().collect()).collect();
    part1(&antenna_map);
    part2(&antenna_map);
}

fn part1(antenna_map: &Vec<Vec<char>>) {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..antenna_map.len() {
        for y in 0..antenna_map[0].len() {
            let ch = antenna_map[x][y];
            if ch != '.' {
                antennas.entry(ch).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    for (_antenna, locations) in antennas {
        for i in 0..locations.len() {
            for j in 0..locations.len() {
                if i != j {
                    let x1: i32 =
                        locations[i].0 as i32 + 2 * (locations[j].0 as i32 - locations[i].0 as i32);
                    let y1: i32 =
                        locations[i].1 as i32 + 2 * (locations[j].1 as i32 - locations[i].1 as i32);
                    let x2: i32 =
                        locations[j].0 as i32 + 2 * (locations[i].0 as i32 - locations[j].0 as i32);
                    let y2: i32 =
                        locations[j].1 as i32 + 2 * (locations[i].1 as i32 - locations[j].1 as i32);

                    //println!("for antenna {0} {1} and {2} {3} checking antinodes {x1} {y1} and {x2} {y2}", locations[i].0, locations[i].1, locations[j].0, locations[j].1);

                    if x1 >= 0
                        && y1 >= 0
                        && x1 < antenna_map.len() as i32
                        && y1 < antenna_map[0].len() as i32
                    {
                        antinodes.insert((x1 as usize, y1 as usize));
                    }
                    if x2 >= 0
                        && y2 >= 0
                        && x2 < antenna_map.len() as i32
                        && y2 < antenna_map[0].len() as i32
                    {
                        antinodes.insert((x2 as usize, y2 as usize));
                    }
                }
            }
        }
    }
    println!("Part 1: {0} total antinodes", antinodes.len());
}
fn part2(antenna_map: &Vec<Vec<char>>) {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..antenna_map.len() {
        for y in 0..antenna_map[0].len() {
            let ch = antenna_map[x][y];
            if ch != '.' {
                antennas.entry(ch).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    for (_antenna, locations) in antennas {
        for i in 0..locations.len() {
            for j in 0..locations.len() {
                if i != j {
                    for k in 0.. {
                        let (loc1_x, loc1_y) = (locations[i].0 as i32, locations[i].1 as i32);
                        let (loc2_x, loc2_y) = (locations[j].0 as i32, locations[j].1 as i32);

                        let x1 = loc1_x + k * (loc2_x - loc1_x);
                        let y1 = loc1_y + k * (loc2_y - loc1_y);
                        let x2 = loc2_x + k * (loc1_x - loc2_x);
                        let y2 = loc2_y + k * (loc1_y - loc2_y);

                        let bounds_x = antenna_map.len() as i32;
                        let bounds_y = antenna_map[0].len() as i32;

                        let valid1 = x1 >= 0 && y1 >= 0 && x1 < bounds_x && y1 < bounds_y;
                        let valid2 = x2 >= 0 && y2 >= 0 && x2 < bounds_x && y2 < bounds_y;

                        if valid1 {
                            antinodes.insert((x1 as usize, y1 as usize));
                        }
                        if valid2 {
                            antinodes.insert((x2 as usize, y2 as usize));
                        }

                        if !valid1 && !valid2 {
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {0} total antinodes", antinodes.len());
}
