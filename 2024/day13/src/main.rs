use std::fs;

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn get_minimal_moves(&self) -> Option<(usize, usize)> {
        let (a1, a2) = self.button_a;
        let (b1, b2) = self.button_b;
        let (p1, p2) = self.prize;

        // Solve for x and y such that a1 * x + b1 * y = p1 and a2 * x + b2 * y = p2

        if let Some((x, y)) = solve_diophantine(a1, b1, p1, a2, b2, p2) {
            if x >= 0 && y >= 0 {
                Some((x as usize, y as usize))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_minimal_ticket_to_win(&self) -> Option<usize> {
        self.get_minimal_moves()
            .map(|(moves_a, moves_b)| moves_a * 3 + moves_b)
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn solve_diophantine(a1: i64, b1: i64, p1: i64, a2: i64, b2: i64, p2: i64) -> Option<(i64, i64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        return None;
    }

    let (gcd_a1_b1, _, _) = extended_gcd(a1, b1);
    if p1 % gcd_a1_b1 != 0 {
        return None;
    }
    let (gcd_a2_b2, _, _) = extended_gcd(a2, b2);
    if p2 % gcd_a2_b2 != 0 {
        return None;
    }

    let x = p1 * b2 - p2 * b1;
    let y = p2 * a1 - p1 * a2;

    if x % det != 0 || y % det != 0 {
        return None;
    }

    Some((x / det, y / det))
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let machines = parse_machines(&input);

    println!("Part 1: {0}", part1(&machines));
    println!("Part 2: {0}", part2(&machines));
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

fn parse_coordinates(input: &str) -> (i64, i64) {
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

fn part2(machines: &Vec<Machine>) -> usize {
    machines
        .iter()
        .map(|machine| Machine {
            button_a: machine.button_a,
            button_b: machine.button_b,
            prize: (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            ),
        })
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
        // button_a: (54, 99), button_b: (95, 17), prize: (13421, 11246)
        let machine = Machine {
            button_a: (54, 99),
            button_b: (95, 17),
            prize: (13421, 11246),
        };
        assert_eq!(machine.get_minimal_moves(), Some((99, 85)));
        //Machine { button_a: (43, 84), button_b: (42, 25), prize: (6289, 8007) },
        let machine = Machine {
            button_a: (43, 84),
            button_b: (42, 25),
            prize: (6289, 8007),
        };
        assert_eq!(machine.get_minimal_moves(), Some((73, 75)));
        let machine = Machine {
            button_a: (49, 13),
            button_b: (28, 57),
            prize: (10000000004561, 10000000011243),
        };
        assert_eq!(
            machine.get_minimal_moves(),
            Some((119390695737, 148209139766))
        );
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
