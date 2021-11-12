use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    let _m: i32 = split.next().unwrap().parse().unwrap();
    let k: i32 = split.next().unwrap().parse().unwrap();

    // Store desired apartment size
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut desired_apt_sizes = line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    desired_apt_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Put all available apartment prices into a BTreeMap with price as the key and the
    // number of occurances as the value
    let mut available_apt_sizes = BTreeMap::new();
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for apt in line.split_whitespace() {
        let apt = apt.parse::<i32>().unwrap();
        let p = available_apt_sizes.entry(apt).or_insert(0);
        *p += 1;
    }

    // Manually lock stdout for performance improvements, see:
    // https://nnethercote.github.io/perf-book/io.html#locking
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let mut cnt = 0;
    for d in desired_apt_sizes {
        // Select the range of available apartments +/- delta "k"
        cnt += match available_apt_sizes.range(d-k..=d+k).next() {
            // If something is found, decrement the BTree counter and return 1
            Some((&p, &v)) => {
                match v {
                    // If the counter is 1, there will be nothing left after this
                    // iteration, so remove the key from the BTreeMap
                    1 => available_apt_sizes.remove(&p),
                    // Do the actual counter decrement
                    _ => available_apt_sizes.insert(p, v - 1),
                };
                1
            },
            // If something nothing is found, return 0
            None => 0
        };
    }

    writeln!(lock, "{}", cnt).unwrap();
}
