use std::fs::read_to_string;

fn main() {
    let mut ingredient_ranges: Vec<(usize, usize)> = vec![];
    let mut fresh_ingredients_count = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        if line.contains("-") {
            let parts = line.split_once("-").unwrap();
            ingredient_ranges.push((parts.0.parse().unwrap(), parts.1.parse().unwrap()));
        } else {
            if let Ok(ingredient_id) = line.parse::<usize>() {
                if ingredient_ranges
                    .iter()
                    .any(|range| ingredient_id >= range.0 && ingredient_id <= range.1)
                {
                    fresh_ingredients_count += 1;
                }
            }
        }
    }
    dbg!(fresh_ingredients_count);
}
