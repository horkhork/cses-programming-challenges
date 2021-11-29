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

    // Parse the array of numbers
    let towers: BTreeMap<usize, usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        //.inspect(|v| debugln!("v{:?}", v))
        .fold(BTreeMap::new(), |mut towers, i| match towers.range(i..usize::MAX).next() {
            Some((&k, &v)) => {
                debug!("found tower for '{}' with prev '{}' and count {}; ", i, k, v);
                if v == 1 {
                    debugln!("Remove '{}'", k);
                    towers.remove(&k);
                } else {
                    debugln!("Decr '{}' {}", k, v - 1);
                    towers.insert(k, v - 1);
                }
                *towers.entry(i).or_insert(0) += 1;
                towers
            },
            None => {
                debugln!("new tower starting at {}", i);
                *towers.entry(i).or_insert(0) += 1;
                towers
            }
        });
    debugln!("{:?}", towers);
    println!("{}", towers.keys().len());
}
