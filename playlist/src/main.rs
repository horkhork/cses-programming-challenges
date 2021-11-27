use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();

    // Parse the first line, unused
    let (_n, ) = match lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<i32>>()[..]
    {
        [n, ] => (n, ),
        _ => panic!("First line not valid"),
    };

    // Keep track of each number in the array and it's most recent index in the list
    let mut indexes: BTreeMap<i32, usize> = BTreeMap::new();

    let mut lb = 0;
    let mut max = 1;
    // Parse the array of numbers
    let mut it: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        //.inspect(|v| println!("v{:?}", v))
        .fold(1, |i, v| {
            //println!("i{}", i);
            if let Some(x) = indexes.get(&v) {
                //println!("Some: {}@{}", v, x);
                lb += 1;
            } else {
                //println!("Not some");
                println!("range {}-{}={}", i, lb, i-lb);
            };
            indexes.insert(v, i as usize);
            i + 1
        });
    println!("It {:?}", it);
    println!("Indexes {:?}", indexes);
}
