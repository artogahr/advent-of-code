use std::fs::read_to_string;

fn main() {
    println!("test");
    let ranges: Vec<(usize, usize)> = read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|range| range.trim().split_once('-').unwrap())
        .map(|range| (range.0.parse().unwrap(), range.1.parse().unwrap()))
        .collect();
    let mut total = 0;

    for range in ranges {
        for i in range.0..=range.1 {
            let i_str = i.to_string();
            // println!("Checking {i}");
            if i_str.len() % 2 == 0 {
                let i_strs = i_str.split_at(i_str.len() / 2);
                // dbg!(i_strs);
                if i_strs.0 == i_strs.1 {
                    total += i;
                }
            }
        }
    }

    dbg!(total);
}
