use std::fs;

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),

    prize: (usize, usize),
}

impl Machine {
    fn get_minimal_moves(&self) -> Option<(usize, usize)> {
        let mut found_combinations: Vec<(usize, usize)> = Vec::new();

        for a in (0..100).rev() {
            for b in 0..=100 {
                /*  println!("({0} {1})*{2} + ({3} {4})*{5} = ({6} {7}), looking for ({8} {9})",
                    self.button_a.0, self.button_a.1, a,
                    self.button_b.0, self.button_b.0, b,
                    self.button_a.0 * a , self.button_b.0 * b,
                    self.prize.0, self.prize.1
                );*/

                if self.button_a.0 * a + self.button_b.0 * b > self.prize.0
                    || self.button_a.1 * a + self.button_b.1 * b > self.prize.1
                {
                    break;
                }

                if self.button_a.0 * a + self.button_b.0 * b == self.prize.0
                    && self.button_a.1 * a + self.button_b.1 * b == self.prize.1
                {
                    //println!("Found {a} {b}");
                    found_combinations.push((a, b));
                }
            }
        }

        //dbg!(&found_combinations);

        let mut answer_index = 0;
        let mut min_combination_answer = usize::MAX;

        for i in 0..found_combinations.len() {
            if found_combinations[i].0 * 3 + found_combinations[i].1 < min_combination_answer {
                min_combination_answer = found_combinations[i].0 * 3 + found_combinations[i].1;
                answer_index = i;
            }
        }

        //dbg!(&found_combinations[answer_index]);

        if found_combinations.len() > 0 {
            Some(found_combinations[answer_index])
        } else {
            None
        }
    }

    fn get_minimal_ticket_to_win(&self) -> Option<usize> {
        let minimal_moves = &self.get_minimal_moves();
        match minimal_moves {
            Some(moves) => Some(moves.0 * 3 + moves.1),
            None => None,
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let machines = parse_machines(&input);

    println!("Part 1: {0}", part1(&machines));
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    let mut current_machine = Machine {
        button_a: (0, 0),
        button_b: (0, 0),
        prize: (0, 0),
    };

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(":").collect();
        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "Button A" => {
                current_machine.button_a = parse_coordinates(value);
            }
            "Button B" => {
                current_machine.button_b = parse_coordinates(value);
            }
            "Prize" => {
                current_machine.prize = parse_coordinates(value);
                machines.push(current_machine);
                current_machine = Machine {
                    button_a: (0, 0),
                    button_b: (0, 0),
                    prize: (0, 0),
                };
            }
            _ => {}
        }
    }

    machines
}

fn parse_coordinates(input: &str) -> (usize, usize) {
    let parts: Vec<&str> = input.split(",").collect();

    let x = parts[0]
        .trim()
        .trim_start_matches("X=")
        .trim_start_matches("X+")
        .parse()
        .unwrap();

    let y = parts[1]
        .trim()
        .trim_start_matches("Y=")
        .trim_start_matches("Y+")
        .parse()
        .unwrap();

    (x, y)
}

fn part1(machines: &Vec<Machine>) -> usize {
    machines
        .iter()
        .filter_map(|machine| machine.get_minimal_ticket_to_win())
        .sum()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn check_correct_machine() {
        /*  Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400 */
        let machine = Machine {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };
        assert_eq!(machine.get_minimal_moves(), Some((80, 40)));

        /*  Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450 */
        let machine = Machine {
            button_a: (17, 86),
            button_b: (84, 37),
            prize: (7870, 6450),
        };
        assert_eq!(machine.get_minimal_moves(), Some((38, 86)));
    }

    #[test]
    fn check_incorrect_machine() {
        /*  Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176*/

        let machine = Machine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748, 12176),
        };
        assert_eq!(machine.get_minimal_moves(), None);
    }
}
