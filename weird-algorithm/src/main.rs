use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut a: i64 = split.next().unwrap().parse().unwrap();

    // Problem:
    // Consider an algorithm that takes as input a positive integer n. If n is even, the algorithm
    // divides it by two, and if n is odd, the algorithm multiplies it by three and adds one. The
    // algorithm repeats this, until n is one.

    let mut ret = Vec::new();
    while a != 1 {
        ret.push(a.to_string());
        a = match a % 2 {
            0 => a / 2,
            _ => (a * 3) + 1
        }
    }
    ret.push(a.to_string());
    println!("{}", ret.join(" "));
}
