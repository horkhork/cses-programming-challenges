use indexmap::IndexSet;
use std::io::{BufRead, BufReader};


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

    let mut children: IndexSet<_> = (1..=n).collect();
    let mut idx = k % n;
    dbg!(k);
    dbg!(idx);
    while let Some(c) = children.swap_take(&idx) {
        dbg!(c);
        idx = (idx + k) % n;
        dbg!(k);
        dbg!(idx);
    }
}
