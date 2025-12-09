use std::fs::read_to_string;

fn main() {
    let mut ingredient_ranges: Vec<(usize, usize)> = vec![];

    let mut fresh_ingredients_count = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        if line.contains("-") {
            let parts = line.split_once("-").unwrap();
            let mut new_range: (usize, usize) =
                (parts.0.parse().unwrap(), parts.1.parse().unwrap());
            // println!("Looking at: {:?}", new_range);

            let mut found_overlap = true;
            while found_overlap {
                found_overlap = false;
                let mut i = 0;
                while i < ingredient_ranges.len() {
                    let range = ingredient_ranges[i];
                    // println!("Checking against: {:?}", range);
                    if !(new_range.1 < range.0 || new_range.0 > range.1) {
                        // println!("Overlapping range found");
                        new_range = (range.0.min(new_range.0), range.1.max(new_range.1));
                        // println!("Removing range: {:?}", ingredient_ranges[i]);
                        ingredient_ranges.remove(i);
                        // println!("Combined range: {:?}", new_range);
                        found_overlap = true;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }

            // println!("Adding range to list: {:?}", new_range);
            ingredient_ranges.push(new_range);
        } else {
            continue;
        }
    }

    // dbg!(&ingredient_ranges);

    for range in ingredient_ranges {
        fresh_ingredients_count += range.1 - range.0 + 1;
    }

    dbg!(fresh_ingredients_count);
}
