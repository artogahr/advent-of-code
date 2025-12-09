use std::fs::read_to_string;

fn main() {
    let mut chars_grid: Vec<Vec<char>> = vec![vec![]];
    let mut adj_nums_grid: Vec<Vec<usize>> = vec![vec![]];

    for (i, line) in read_to_string("input.txt").unwrap().lines().enumerate() {
        chars_grid.push(vec![]);
        adj_nums_grid.push(vec![]);
        for char in line.chars() {
            chars_grid[i].push(char);
            adj_nums_grid[i].push(0);
        }
    }

    for (i, line) in chars_grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            for dx in -1_isize..=1 {
                for dy in -1_isize..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let x = i as isize + dx;
                    let y = j as isize + dy;
                    if x >= 0
                        && x < chars_grid.len() as isize - 1
                        && y >= 0
                        && y < line.len() as isize
                    {
                        // dbg!(x, y, i, j, line);
                        if chars_grid[x as usize][y as usize] == '@' {
                            adj_nums_grid[i][j] += 1;
                        }
                    }
                }
            }
        }
    }

    // for (i, line) in chars_grid.iter().enumerate() {
    //     for (j, char) in line.iter().enumerate() {
    //         print!("{char}");
    //     }
    //     println!();
    // }
    for (i, line) in chars_grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if adj_nums_grid[i][j] < 4 && *char == '@' {
                print!("x");
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    // for (i, line) in adj_nums_grid.iter().enumerate() {
    //     for (j, count) in line.iter().enumerate() {
    //         if chars_grid[i][j] == '@' {
    //             print!("{count}");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut total_count = 0;
    for (i, line) in chars_grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if chars_grid[i][j] == '@' {
                if adj_nums_grid[i][j] < 4 {
                    // println!("Found roll at {i} {j} ");
                    total_count += 1;
                }
            }
        }
    }
    dbg!(total_count);
}
