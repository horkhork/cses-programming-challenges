use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _: i32 = split.next().unwrap().parse().unwrap();

    let mut numbers = BTreeMap::new();

    // Put all numbers into a BTreeMap with num as the key and the
    // number of occurances as the value
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for n in line.split_whitespace() {
        let n = n.parse::<i32>().unwrap();
        let e = numbers.entry(n).or_insert(0);
        *e += 1;
    }

    // Manually lock stdout for performance improvements, see:
    // https://nnethercote.github.io/perf-book/io.html#locking
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let val = numbers.keys().len();
    writeln!(lock, "{}", val).unwrap();
}
