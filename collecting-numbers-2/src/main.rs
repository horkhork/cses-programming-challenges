use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

// 5 3
// 4 2 1 5 3 -> 3
// 2 3 -> 4 1 2 5 3 -> 2
// 1 5 -> 3 1 2 5 4 -> 3
// 2 3 -> 3 2 1 5 4 -> 4

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let (n, m) = match line.split_whitespace().filter_map(|v| v.parse::<i32>().ok()).collect::<Vec<i32>>()[..] {
        [n, m] => (n, m),
        _ => panic!("Line not valid: {}", line),
    };

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();

    let mut arr: Vec<i64> = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();

    //println!("{}", cnt);
}

fn collect_numbers(arr: &Vec<i64>) -> usize {
    let mut set: BTreeSet<i64> = BTreeSet::new();
    arr.iter().map(|x| {
        set.insert(*x);
        match set.contains(&(x - 1)) {
            true => 0,
            false => 1,
        }
    })
    .sum()
}
