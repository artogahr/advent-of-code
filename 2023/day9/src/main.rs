fn parse_input() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|line: String| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().expect("Failed to parse i32"))
                .collect()
        })
        .collect()
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let trees = parse_trees(input);
    let mut sum: i32 = 0;
    for mut tree in trees {
        let len = tree.len();
        let mut last_num_to_add: i32 = 0;
        for sequence in tree.iter_mut().take(len - 1).rev() {
            sequence.push(last_num_to_add);
            last_num_to_add = sequence.iter().rev().take(2).sum();
        }
        sum += last_num_to_add;
    }
    sum
}

fn parse_trees(input: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut trees: Vec<Vec<Vec<i32>>> = Default::default();
    for line in input {
        let mut tree: Vec<Vec<i32>> = Default::default();
        tree.push(line.clone());
        let mut next_line = parse_inbetween_values(&line);
        loop {
            if !next_line.iter().all(|f| *f == 0) {
                tree.push(next_line.clone());
                next_line = parse_inbetween_values(&next_line);
            } else {
                tree.push(next_line.clone());
                break;
            }
        }
        trees.push(tree);
    }
    trees
}

fn parse_inbetween_values(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Default::default();
    for (i, num) in numbers[0..numbers.len() - 1].iter().enumerate() {
        result.push(numbers[i + 1] - num);
    }
    result
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let trees = parse_trees(input);
    let mut sum: i32 = 0;
    for mut tree in trees {
        let len = tree.len();
        let mut first_num_to_add: i32 = 0;
        for sequence in tree.iter_mut().take(len - 1).rev() {
            sequence.insert(0, sequence[0] - first_num_to_add);

            first_num_to_add = sequence.iter().nth(1).unwrap() - first_num_to_add;
        }
        sum += first_num_to_add;
    }
    sum
}

fn main() {
    let solution = advent::new(parse_input).part(part1).part(part2).build();
    solution.cli()
}
