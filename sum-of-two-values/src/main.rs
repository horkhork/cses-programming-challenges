use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    let x: i32 = split.next().unwrap().parse().unwrap();

    let mut values: BTreeSet<i32> = BTreeSet::new();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for val in line.split_whitespace() {
        values.insert(val.parse::<i32>().unwrap());
    }
    println!("values: {:?}", values);

    println!("{}", -1);
}
