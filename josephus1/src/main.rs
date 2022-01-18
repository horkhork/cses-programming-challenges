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
        [n, k] => (n, k),
        [n] => (n, 2),
        _ => panic!("First line not valid"),
    };

    let mut children: IndexSet<_> = (1..=n).collect();
    dbg!(&children);
    let mut idx = k % n;
    dbg!(n);
    dbg!(k);
    dbg!(idx);
    while let Some(child) = children.swap_remove_index(idx) {
        dbg!(child);
        print!("{} ", child);
        let len = children.len();
        dbg!(len);
        idx = if len == 0 { 0 } else { (idx + k) % len };

        dbg!(k);
        dbg!(idx);
    }
    println!();
}
