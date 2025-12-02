use std::fs::read_to_string;

fn main() {
    let ranges: Vec<(usize, usize)> = read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|range| range.trim().split_once('-').unwrap())
        .map(|range| (range.0.parse().unwrap(), range.1.parse().unwrap()))
        .collect();

    let mut total = 0;

    for range in ranges {
        'main: for i in range.0..=range.1 {
            let i_str = i.to_string();
            // print!("Checking {i}\t");

            'chunks: for chunk_size in 1..=i_str.len() / 2 {
                if i_str.len() % chunk_size == 0 {
                    // print!("found potential entry for chunk size {chunk_size}\t");

                    let chunks: Vec<Vec<u8>> = i_str
                        .as_bytes()
                        .chunks(chunk_size)
                        .map(|c| c.to_vec())
                        .collect();
                    // dbg!(&chunks);

                    for chunk_pair in chunks.windows(2) {
                        // println!("Checking {:?} and {:?}", chunk_pair[0], chunk_pair[1]);
                        if chunk_pair[0] != chunk_pair[1] {
                            // println!("{i} is not valid");
                            continue 'chunks;
                        }
                    }

                    // println!("\tfound valid joke entry: {i}");
                    total += i;
                    continue 'main;
                }
            }
        }
    }

    dbg!(total);
}
