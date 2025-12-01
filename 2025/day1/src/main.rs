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
        println!("The dial is rotated {} to point at {}", amount, self.arrow);
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
        dial.turn(sign * amount);
        if dial.arrow == 0 {
            count += 1;
        }
    }
    println!("{0}", count);
}
