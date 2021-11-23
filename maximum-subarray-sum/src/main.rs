use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut values = Vec::new();
    for val in line.split_whitespace() {
        let val = val.parse::<i64>().unwrap();
        values.push(val);
    }

    let best = values
        .iter()
        .fold((i64::MIN, 0), |(mut best, mut current), x| {
            if current <= 0 {
                current = *x;
            } else {
                current += x;
            }
            if current > best {
                best = current;
            }
            (best, current)
        });
    println!("{:?}", best.0);
}
