use std::io;
//use std::io::Write;
fn main() {
    let mut score: i32 = 0;
    let mut current_score: i32 = 0;
    loop{
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("couldn't read");
        if str.trim().is_empty() {
            break;
        }
        let mut values = str.split_whitespace();
        let expected_move = values.next().expect("No string for expected move");
        let next_move = values.next().expect("No string for next move");
        match expected_move {
            "A" => match next_move {
                "X" => current_score = 1 + 3,
                "Y" => current_score = 2 + 6,
                "Z" => current_score = 3 + 0,
                _   => continue,
            }
            "B" => match next_move {
                "X" => current_score = 1 + 0,
                "Y" => current_score = 2 + 3,
                "Z" => current_score = 3 + 6,
                _   => continue,
            }
            "C" => match next_move {
                "X" => current_score = 1 + 6,
                "Y" => current_score = 2 + 0,
                "Z" => current_score = 3 + 3,
                _   => continue,
            }
            _ => println!("{expected_move}"),
        }
        score += current_score;
        current_score = 0;
    }
    println!("{score}");
}
