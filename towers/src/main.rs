use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

#[allow(unused_macros)]
#[cfg(feature = "debug")]
macro_rules! debugln {
    ($( $args:expr ),*) => { println!( $( $args ),* ) }
}
#[allow(unused_macros)]
#[cfg(feature = "debug")]
macro_rules! debug {
    ($( $args:expr ),*) => { print!( $( $args ),* ) }
}

#[allow(unused_macros)]
#[cfg(not(feature = "debug"))]
macro_rules! debugln {
    ($( $args:expr ),*) => {}
}
#[allow(unused_macros)]
#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($( $args:expr ),*) => {}
}

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
    let mut towers: BTreeMap<i32, usize> = BTreeMap::new();

    // Parse the array of numbers
    let _: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .inspect(|v| debugln!("v{:?}", v))
        .map(|i| match towers.range(i..i32::MAX).next() {
            Some((&k, &v)) => {
                debugln!("found tower for '{}' with prev '{}' and count {}", i, k, v);
                0
            },
            None => {
                debugln!("new tower starting at {}", i);
                *towers.entry(i).or_insert(0) += 1;
                1
            }
        })
        .collect();
    debugln!("{:?}", towers);
}
