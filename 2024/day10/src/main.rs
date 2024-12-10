use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = BufReader::new(File::open("input.txt").unwrap());
    let tg_map: Vec<Vec<u32>> = f
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    part1(&tg_map);
    part2(&tg_map);
}

fn part1(tg_map: &Vec<Vec<u32>>) {
    let mut result = 0;
    for (x, line) in tg_map.iter().enumerate() {
        for (y, number) in line.iter().enumerate() {
            if *number == 0 {
                let score = trails(tg_map, x as i32, y as i32);
                result += score;
            }
        }
    }
    println!("Part 1: Sum of the scores is {result}");
}

fn trails(tg_map: &Vec<Vec<u32>>, x: i32, y: i32) -> usize {
    let mut reachable_ends: HashSet<(i32, i32)> = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn search_recursively(
        tg_map: &Vec<Vec<u32>>,
        reachable_ends: &mut HashSet<(i32, i32)>,
        x: i32,
        y: i32,
        directions: &[(i32, i32)],
    ) {
        //println!("Looking at {x} {y} : {0}", tg_map[x as usize][y as usize]);
        if tg_map[x as usize][y as usize] == 9 {
            reachable_ends.insert((x, y));
            return;
        }
        for dir in directions {
            if x + dir.0 >= 0
                && y + dir.1 >= 0
                && x + dir.0 < tg_map.len() as i32
                && y + dir.1 < tg_map[0].len() as i32
                && tg_map[(x + dir.0) as usize][(y + dir.1) as usize]
                    == tg_map[x as usize][y as usize] + 1
            {
                search_recursively(&tg_map, reachable_ends, x + dir.0, y + dir.1, directions);
            }
        }
    }

    search_recursively(&tg_map, &mut reachable_ends, x, y, &directions);

    reachable_ends.len()
}

fn part2(tg_map: &Vec<Vec<u32>>) {
    let mut result = 0;
    for (x, line) in tg_map.iter().enumerate() {
        for (y, number) in line.iter().enumerate() {
            if *number == 0 {
                let score = trails2(tg_map, x as i32, y as i32);
                result += score;
            }
        }
    }
    println!("Part 2: Sum of the scores is {result}");
}

fn trails2(tg_map: &Vec<Vec<u32>>, x: i32, y: i32) -> usize {
    let mut reachable_ends: Vec<(i32, i32)> = Vec::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn search_recursively(
        tg_map: &Vec<Vec<u32>>,
        reachable_ends: &mut Vec<(i32, i32)>,
        x: i32,
        y: i32,
        directions: &[(i32, i32)],
    ) {
        //println!("Looking at {x} {y} : {0}", tg_map[x as usize][y as usize]);
        if tg_map[x as usize][y as usize] == 9 {
            reachable_ends.push((x, y));
            return;
        }
        for dir in directions {
            if x + dir.0 >= 0
                && y + dir.1 >= 0
                && x + dir.0 < tg_map.len() as i32
                && y + dir.1 < tg_map[0].len() as i32
                && tg_map[(x + dir.0) as usize][(y + dir.1) as usize]
                    == tg_map[x as usize][y as usize] + 1
            {
                search_recursively(&tg_map, reachable_ends, x + dir.0, y + dir.1, directions);
            }
        }
    }

    search_recursively(&tg_map, &mut reachable_ends, x, y, &directions);

    reachable_ends.len()
}
