use std::cmp;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();

    // Parse the first line, unused
    let (_n,) = match lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<i32>>()[..]
    {
        [n] => (n,),
        _ => panic!("First line not valid"),
    };

    // Keep track of each number in the array and it's most recent index in the list
    let mut indexes: BTreeMap<i32, usize> = BTreeMap::new();

    // Parse the array of numbers
    let (_, max): (i32, i32) = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        //.inspect(|v| println!("v{:?}", v))
        .enumerate()
        .fold((0, 0), |(mut lb, mut max), (i, v)| {
            // enumerated indexes are 1-based to be inclusive for the current item.
            let i = i + 1;
            //print!("{:>5}: value '{}'; ", i, v);
            if let Some(prev) = indexes.get(&v) {
                //print!("update lb from {} ", lb);
                lb = cmp::max(lb, *prev as i32);
                //print!("to {}; ", lb);
            };
            max = cmp::max(max, i as i32 - lb);
            //println!("range {}-{}={}", i, lb, i as i32 - lb);
            indexes.insert(v, i as usize);
            (lb, max)
        });
    println!("{}", max);
}
