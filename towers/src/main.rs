use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::ops::Bound::{Excluded, Included};

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
    ($( $args:expr ),*) => {};
}
#[allow(unused_macros)]
#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($( $args:expr ),*) => {};
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

    // Parse the array of numbers, using a BTreeMap to keep track of the number
    // of towers needed. For each number N, look for an existing tower with the
    // closest key less than the current N. If found, decrement the value there,
    // removing the key altogether if the value would be zero. Create a new
    // tower for N, incrementing the value if a tower already exists.
    let towers: BTreeMap<usize, usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        //.inspect(|v| debugln!("v{:?}", v))
        .fold(BTreeMap::new(), |mut towers, n| {
            // For each value N find the key corresponding to the next-biggest value
            match towers.range((Excluded(n), Included(usize::MAX))).next() {
                Some((&k, &v)) => {
                    debug!(
                        "found tower for '{}' with prev '{}' and count {}; ",
                        n, k, v
                    );
                    if v == 1 {
                        debugln!("Remove '{}'", k);
                        towers.remove(&k);
                    } else {
                        debugln!("Decr '{}' {}", k, v - 1);
                        towers.insert(k, v - 1);
                    }
                    *towers.entry(n).or_insert(0) += 1;
                    towers
                }
                None => {
                    debugln!("new tower starting at {}", n);
                    *towers.entry(n).or_insert(0) += 1;
                    towers
                }
            }
        });
    debugln!("{:?}", towers);
    // The values here sums up to the total number of towers needed
    println!("{}", towers.iter().fold(0, |acc, (_, v)| acc + v));
}
