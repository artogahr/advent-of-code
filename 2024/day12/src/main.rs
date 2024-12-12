use std::{fs, usize};

#[derive(Copy, Clone)]

struct Plot {
    plant: char,
    region_id: Option<usize>,
    x: usize,
    y: usize,
}

struct Region {
    plots: Vec<Plot>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter_len(&self) -> usize {
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;

        self.plots.iter().for_each(|plot| {
            max_x = plot.x.max(max_x);
            max_y = plot.y.max(max_y);
        });

        let mut empty_garden: Vec<Vec<Plot>> = Vec::new();

        for x in 0..max_x + 1 {
            empty_garden.push(Vec::new());
            for y in 0..max_y + 1 {
                empty_garden[x].push(Plot {
                    plant: '.',
                    region_id: None,
                    x,
                    y,
                });
            }
        }

        for plot in &self.plots {
            empty_garden[plot.x][plot.y] = plot.clone();
        }
        //println!("Starting crawling");
        //print_garden(&empty_garden);
        let found_edges = crawl(&mut empty_garden, self.plots[0].x, self.plots[0].y);

        fn crawl(garden: &mut Vec<Vec<Plot>>, x: usize, y: usize) -> usize {
            //println!("Crawling {x} {y}");
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut found_edges: usize = 0;
            garden[x][y].region_id = None;
            if x == 0 {
                found_edges += 1;
            }
            if y == 0 {
                found_edges += 1;
            }
            if x == garden.len() - 1 {
                found_edges += 1;
            }
            if y == garden[0].len() - 1 {
                found_edges += 1;
            }
            for dir in directions {
                let x: i32 = x as i32 + dir.0;
                let y: i32 = y as i32 + dir.1;

                if x >= 0 && y >= 0 && x < garden.len() as i32 && y < garden[0].len() as i32 {
                    if garden[x as usize][y as usize].plant == '.' {
                        found_edges += 1;
                    } else if !garden[x as usize][y as usize].region_id.is_none() {
                        found_edges += crawl(garden, x as usize, y as usize);
                    }
                }
            }
            //println!("Found eges {found_edges}");
            found_edges
        }

        ////print_garden(&empty_garden);
        found_edges
    }

    fn sides_count(&self) -> usize {
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        let mut found_sides = 0;

        self.plots.iter().for_each(|plot| {
            max_x = plot.x.max(max_x);
            max_y = plot.y.max(max_y);
        });

        let mut empty_garden: Vec<Vec<Plot>> = Vec::new();

        for x in 0..max_x + 3 {
            empty_garden.push(Vec::new());
            for y in 0..max_y + 3 {
                empty_garden[x].push(Plot {
                    plant: '.',
                    region_id: None,
                    x,
                    y,
                });
            }
        }

        for plot in &self.plots {
            empty_garden[plot.x + 1][plot.y + 1] = plot.clone();
        }
        //println!("Starting searching for sides");
        //print_garden(&empty_garden);
        let plant = self.plots[0].plant;
        let mut currently_looking;
        // Look line by line
        // Look at horizontal lines, look at the top of the line
        for x in 0..empty_garden.len() {
            currently_looking = false;
            for y in 0..empty_garden[0].len() {
                if empty_garden[x][y].plant == plant && empty_garden[x - 1][y].region_id.is_none() {
                    if !currently_looking {
                        currently_looking = true;
                        found_sides += 1;
                    }
                } else {
                    currently_looking = false;
                }
            }
        }
        // Look at horizontal lines, look at the bottom of the line
        for x in 0..empty_garden.len() {
            currently_looking = false;
            for y in 0..empty_garden[0].len() {
                if empty_garden[x][y].plant == plant && empty_garden[x + 1][y].region_id.is_none() {
                    if !currently_looking {
                        currently_looking = true;
                        found_sides += 1;
                    }
                } else {
                    currently_looking = false;
                }
            }
        }
        // Look at vertical lines, look at the left of the line
        for y in 0..empty_garden[0].len() {
            currently_looking = false;
            for x in 0..empty_garden.len() {
                if empty_garden[x][y].plant == plant && empty_garden[x][y - 1].region_id.is_none() {
                    if !currently_looking {
                        currently_looking = true;
                        found_sides += 1;
                    }
                } else {
                    currently_looking = false;
                }
            }
        }
        // Look at vertical lines, look at the right of the line
        for y in 0..empty_garden[0].len() {
            currently_looking = false;
            for x in 0..empty_garden.len() {
                if empty_garden[x][y].plant == plant && empty_garden[x][y + 1].region_id.is_none() {
                    if !currently_looking {
                        currently_looking = true;
                        found_sides += 1;
                    }
                } else {
                    currently_looking = false;
                }
            }
        }
        //println!("found sides: {found_sides}");
        found_sides
    }
}

fn main() {
    let garden: Vec<Vec<Plot>> = get_input();

    part1(&garden);
    part2(&garden);
}

fn get_input() -> Vec<Vec<Plot>> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(y, plant)| Plot {
                    plant,
                    region_id: None,
                    x,
                    y,
                })
                .collect()
        })
        .collect()
}

fn part1(garden: &Vec<Vec<Plot>>) {
    let mut garden = garden.clone();

    let mut region_id = 0;

    for x in 0..garden.len() {
        for y in 0..garden[0].len() {
            if garden[x][y].region_id.is_none() {
                crawl_garden(&mut garden, x, y, region_id);
                region_id += 1;
            }
        }
    }

    let regions = get_regions(&garden, region_id);

    let result: usize = regions
        .iter()
        .map(|region| region.area() * region.perimeter_len())
        //.inspect(|num| println!("Found area with total {num}"))
        .sum();

    println!("Part 1: {result}");
}

fn part2(garden: &Vec<Vec<Plot>>) {
    let mut garden = garden.clone();

    let mut region_id = 0;

    for x in 0..garden.len() {
        for y in 0..garden[0].len() {
            if garden[x][y].region_id.is_none() {
                crawl_garden(&mut garden, x, y, region_id);
                region_id += 1;
            }
        }
    }

    let regions = get_regions(&garden, region_id);

    let result: usize = regions
        .iter()
        .map(|region| region.area() * region.sides_count())
        //.inspect(|num| println!("Found area with total {num}"))
        .sum();

    println!("Part 2: {result}");
}
fn crawl_garden(garden: &mut Vec<Vec<Plot>>, x: usize, y: usize, region_id: usize) {
    fn crawl(garden: &mut Vec<Vec<Plot>>, x: usize, y: usize, region_id: usize) {
        let current_plant = garden[x][y].plant;

        garden[x][y].region_id = Some(region_id);

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dir in directions {
            let x: i32 = x as i32 + dir.0;
            let y: i32 = y as i32 + dir.1;

            if x >= 0
                && y >= 0
                && x < garden.len() as i32
                && y < garden[0].len() as i32
                && garden[x as usize][y as usize].plant == current_plant
                && garden[x as usize][y as usize].region_id.is_none()
            {
                crawl(garden, x as usize, y as usize, region_id);
            }
        }
    }

    crawl(garden, x, y, region_id);
}

fn get_regions(garden: &Vec<Vec<Plot>>, region_count: usize) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    for i in 0..region_count {
        let mut region: Region = Region { plots: Vec::new() };
        for line in garden {
            for plot in line {
                if plot.region_id == Some(i) {
                    region.plots.push(*plot);
                }
            }
        }
        regions.push(region);
    }
    regions
}

fn print_garden(garden: &Vec<Vec<Plot>>) {
    println!("---------------------------");
    for x in 0..garden.len() {
        for y in 0..garden[0].len() {
            print!(
                "{0}-{1} ",
                garden[x][y].plant,
                garden[x][y].region_id.unwrap_or(9)
            );
        }

        println!();
    }
}
