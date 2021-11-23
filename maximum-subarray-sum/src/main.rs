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

    let mut best_sum = i64::MIN;
    let mut current_sum: i64 = 0;
    for x in values {
        if current_sum <= 0 {
            current_sum = x;
        } else {
            current_sum += x;
        }
        if current_sum > best_sum {
            best_sum = current_sum;
        }
    }
    println!("{}", best_sum);
}
