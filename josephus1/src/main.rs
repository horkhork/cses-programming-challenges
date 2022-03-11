use std::collections::BTreeMap;
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

    let mut children: BTreeMap<_, _> = (1..=n).fold(BTreeMap::new(), |mut acc, n| {
        acc.insert(n - 1, n);
        acc
    });
    dbg!(&children);
    let mut idx = k % n;
    dbg!(n);
    dbg!(k);
    dbg!(idx);
    let mut len = n;
    let mut prevlen = n - 1;
    while let Some(child) = children.remove(&idx) {
        print!("{} ", child);
        len -= 1;

        dbg!(child);
        dbg!(len);
        idx = if len == 0 { 0 } else { idx + k };
        if idx > len {
            println!("WRAP IDX:{} LEN:{}", idx, len);
            idx = idx % prevlen;
            children = children
                .values()
                .enumerate()
                .fold(BTreeMap::new(), |mut acc, (i, n)| {
                    acc.insert(i, *n);
                    acc
                });
            prevlen = children.len() - 1;
        dbg!(&children);
        } else {
            idx += 1;
        }

        dbg!(idx);
    }
    println!();
}
