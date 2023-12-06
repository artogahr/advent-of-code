fn parse_input() -> Vec<String> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct ConvertMap {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

fn part1(almanac: Vec<String>) -> u64 {
    let mut result: Vec<Vec<ConvertMap>> = Vec::new();
    let mut current_subvector: Vec<ConvertMap> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();

    for (i, line) in almanac.iter().enumerate() {
        if i == 0 {
            seeds = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
        } else if line.is_empty() {
            // Start a new subvector when an empty string is encountered
            // Names of the subvectors are irrelevant, they go in order
            if !current_subvector.is_empty() {
                result.push(current_subvector);
                current_subvector = Vec::new();
            }
        } else {
            // Skip the header (name) of the subvector
            if line.chars().next().unwrap().is_numeric() {
                let line: Vec<u64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();
                let drs = line.first().unwrap();
                let srs = line.get(1).unwrap();
                let len = line.get(2).unwrap();
                let map: ConvertMap = ConvertMap {
                    dest_range_start: *drs,
                    src_range_start: *srs,
                    range_len: *len,
                };
                current_subvector.push(map);
            }
        }
    }

    // Add the last subvector if it's not empty
    if !current_subvector.is_empty() {
        result.push(current_subvector);
    }
    //dbg!(&result);
    let mut locations: Vec<u64> = Vec::new();

    for mut seed in seeds {
        'mapseach: for maps in result.clone() {
            for map in maps {
                if seed >= map.src_range_start && seed < map.src_range_start + map.range_len {
                    seed = map.dest_range_start + seed - map.src_range_start;
                    continue 'mapseach;
                }
            }
        }
        locations.push(seed);
    }
    //dbg!(&locations);

    *locations.iter().min().unwrap()
}

fn part2(almanac: Vec<String>) -> u64 {
    let mut result: Vec<Vec<ConvertMap>> = Vec::new();
    let mut current_subvector: Vec<ConvertMap> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();

    for (i, line) in almanac.iter().enumerate() {
        if i == 0 {
            let line_numbers: Vec<u64> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
            //dbg!(&line_numbers);
            let mut iter = line_numbers.iter();
            while let (Some(&start), Some(&length)) = (iter.next(), iter.next()) {
                let range: Vec<u64> = (start..start + length).collect();
                seeds.extend(range);
            }
            //dbg!(&seeds);
        } else if line.is_empty() {
            // Start a new subvector when an empty string is encountered
            // Names of the subvectors are irrelevant, they go in order
            if !current_subvector.is_empty() {
                result.push(current_subvector);
                current_subvector = Vec::new();
            }
        } else {
            // Skip the header (name) of the subvector
            if line.chars().next().unwrap().is_numeric() {
                let line: Vec<u64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();
                let drs = line.first().unwrap();
                let srs = line.get(1).unwrap();
                let len = line.get(2).unwrap();
                let map: ConvertMap = ConvertMap {
                    dest_range_start: *drs,
                    src_range_start: *srs,
                    range_len: *len,
                };
                current_subvector.push(map);
            }
        }
    }

    // Add the last subvector if it's not empty
    if !current_subvector.is_empty() {
        result.push(current_subvector);
    }
    //dbg!(&result);
    let mut locations: Vec<u64> = Vec::new();
    for mut seed in seeds {
        'mapseach: for maps in &result {
            for &map in maps {
                if seed >= map.src_range_start && seed < map.src_range_start + map.range_len {
                    seed = map.dest_range_start + seed - map.src_range_start;
                    continue 'mapseach;
                }
            }
        }
        locations.push(seed);
    }
    *locations.iter().min().unwrap()
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
