use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();

    // Count of numbers in the list
    let mut last = String::from("");
    let mut cnt = 1;
    let mut m = 0;
    input.read_line(&mut line).unwrap();
    for c in line.chars() {
        //println!("{}", c);
        if c.to_string() == last {
            cnt += 1;
        } else {
            last = c.to_string();
            cnt = 1;
        };
        m = cmp::max(m, cnt);
    }
    println!("{}", m);
}
