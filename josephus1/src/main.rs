use std::collections::BTreeSet;
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
    let (n, k) = match lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<usize>>()[..]
    {
        [n, k] => (n, k + 1),
        [n] => (n, 2),
        _ => panic!("First line not valid"),
    };

    let mut children: BTreeSet<usize> = (1..=n).collect();
    let mut i: usize = 1;
    while !children.is_empty() {
        let mut tmp = children.clone();
        for c in children {
            if i % k == 0 {
                print!("{} ", c);
                tmp.remove(&c);
            }
            i += 1;
        }
        children = tmp;
    }
}
