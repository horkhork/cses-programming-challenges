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

    //let mut lb = 0;
    //let mut max = 1;
    // Parse the array of numbers
    let (_lb, _ub, max): (i32, i32, i32) = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        //.inspect(|v| println!("v{:?}", v))
        .fold((0, 1, 0), |(mut lb, ub, mut max), v| {
            //println!("i{}", i);
            if indexes.contains_key(&v) {
                lb += 1;
            } else {
                max = cmp::max(max, ub - lb);
                //println!("range {}-{}={}", i, lb, i-lb);
            };
            indexes.insert(v, ub as usize);
            (lb, ub + 1, max)
        });
    //println!("It {} {} {}", lb, ub, max);
    //println!("Indexes {:?}", indexes);
    println!("{}", max);
}
