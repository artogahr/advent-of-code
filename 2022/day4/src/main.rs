use std::{io::{self, BufRead}, i32};

fn split_comma(line: String) -> (String, String){
    line.split_once(',').map(|(x, y)| (x.to_string(), y.to_string())).unwrap()
}
fn split_dash(line: String) -> (String, String){
    line.split_once('-').map(|(x, y)| (x.to_string(), y.to_string())).unwrap()
}

fn main() -> io::Result<()> {
    let mut answer = 0;
    let (mut left, mut right, mut rr, mut rl, mut lr, mut ll): (String, String,String,String,String,String);
    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let last_input = line.unwrap();
        // stop reading
        if last_input.is_empty() {
            break;
        }
        (left, right) = split_comma(last_input);
        (ll, lr) = split_dash(left);
        (rl, rr) = split_dash(right);
        let ll = ll.parse::<i32>().unwrap();
        let lr = lr.parse::<i32>().unwrap();
        let rl = rl.parse::<i32>().unwrap();
        let rr = rr.parse::<i32>().unwrap();
        if ll <= rl || lr >= rr || rl <= ll || rr >= lr{
            answer += 1;
        }
    }
    println!("{0}", answer);
    Ok(())
}
