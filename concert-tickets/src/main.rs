use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    let _m: i32 = split.next().unwrap().parse().unwrap();

    let mut tickets = BTreeMap::new();

    // Put all ticket prices into a BTreeMap with price as the key and the
    // number of occurances as the value
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for tik in line.split_whitespace() {
        let tik = tik.parse::<i32>().unwrap();
        let p = tickets.entry(tik).or_insert(0);
        *p += 1;
    }

    // Manually lock stdout for performance improvements, see:
    // https://nnethercote.github.io/perf-book/io.html#locking
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    // Iterate over customer max prices
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for c in line.split_whitespace() {
        let c = c.parse::<i32>().unwrap();

        // This is the magic. The range operator on a BTreeMap will return the
        // next available key in the map in order:
        // https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.range
        // This is an efficient method to either find the exact key, or the next
        // available key. Search for the range 0 to CustomerMaxPrice in reverse
        // order and only take the first element if found.
        let val = match tickets.range(0..=c).rev().next() {
            Some((&k, &v)) => {
                // If something is found, decrement the counter
                match v {
                    // If the counter is 1, there will be nothing left after this
                    // iteration, so remove the key from the BTreeMap
                    1 => tickets.remove(&k),
                    // Do the actual counter decrement
                    _ => tickets.insert(k, v - 1),
                };
                k
            }
            // If something is not found, return -1
            None => -1,
        };

        writeln!(lock, "{}", val).unwrap();
    }
}
