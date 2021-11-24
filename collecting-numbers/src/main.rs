use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let _n: i32 = line.split_whitespace().next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();

    let mut set: BTreeSet<i64> = BTreeSet::new();

    let cnt: usize = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .map(|x| {
            set.insert(x);
            match set.contains(&(x - 1)) {
                true => 0,
                false => 1,
            }
        })
        .sum();

    println!("{}", cnt);
}
