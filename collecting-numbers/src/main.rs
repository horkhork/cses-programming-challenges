use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let n: i32 = line.split_whitespace().next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();

    let values: Vec<i64> = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();

    let mut cnt = 0;
    let mut i = 1;
    while i <= n {
        let mut in_range = false;
        for x in values.clone() {
            if i == x as i32 {
                if !in_range {
                    cnt += 1;
                    in_range = true;
                }
                i += 1;
            }
        }
    }
    println!("{}", cnt);
}
