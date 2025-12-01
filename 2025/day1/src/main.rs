use std::fs::read_to_string;

struct Dial {
    arrow: u32,
}

impl Dial {
    fn new() -> Dial {
        return Dial { arrow: 50 };
    }
    fn turn(&mut self, amount: i32) {
        self.arrow = (self.arrow as i32 + amount).rem_euclid(100) as u32;
        // println!("The dial is rotated {} to point at {}", amount, self.arrow);
    }
}

fn main() {
    let mut count = 0;
    let mut dial = Dial::new();
    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    for line in lines {
        let (dir, amount) = line.split_at(1);
        let amount: i32 = amount.parse().unwrap();
        let sign = if dir == "L" { -1 } else { 1 };
        let amount = amount * sign;
        // println!("Arrow is at {} Will rotate {amount} clicks", dial.arrow);
        let full_rotations = amount / 100;
        // println!("Will do {} full rotations", full_rotations);
        count += full_rotations.abs();
        let remainder = amount - 100 * full_rotations;
        if dial.arrow as i32 + remainder >= 100
            || (dial.arrow as i32 + remainder <= 0 && dial.arrow != 0)
        {
            // println!(
            //     "Remainder is {remainder} and dial is currently at {}\nWill do one more pass through 0",
            //     dial.arrow
            // );
            count += 1;
        }
        dial.turn(amount);
        // println!("count is at {count}\n");
    }
    println!("{0}", count);
}
