use std::collections::{BTreeMap, BTreeSet};
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

    // Parse the first line
    let (x, _n) = match lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<usize>>()[..]
    {
        [x, n] => (x, n),
        _ => panic!("First line not valid"),
    };

    let mut positions: BTreeSet<usize> = BTreeSet::new();
    positions.insert(0);
    positions.insert(x);
    let mut lengths: BTreeMap<usize, usize> = BTreeMap::new();
    lengths.insert(x, 1);

    for p in lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<usize>>()
    {
        positions.insert(p);
        debugln!("add {}: positions:{:?}", p, positions);
        // Get the nearest traffic lights greater than and less than the current position
        let a = positions
            .range((Included(0), Excluded(p)))
            .rev() // need to reverse the range to get one less than current position
            .next()
            .unwrap();
        let b = positions.range((Excluded(p), Included(x))).next().unwrap();
        debugln!("Positions a:{} p:{} b:{}", a, p, b);
        // Update the length counter for the previous span, decrementing by one
        // if there are more than one, removing it entirely otherwise
        let span = b - a;
        if let Some((_, v)) = lengths.remove_entry(&span) {
            if v > 1 {
                lengths.insert(span, v - 1);
            }
        };
        // Update the length counter for the new ranges between the current position
        // and the traffic lights, one less and one greater
        lengths.entry(p - *a).and_modify(|e| *e += 1).or_insert(1);
        lengths.entry(*b - p).and_modify(|e| *e += 1).or_insert(1);
        debugln!("lengths:{:?}", lengths);

        // Print the largest key on this iteration, remember that BTreeMap keys
        // are ordered
        print!("{} ", lengths.keys().last().unwrap());
    }
    debugln!("{:?}\n{:?}", positions, lengths);
}
