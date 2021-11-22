use std::io::{BufRead, BufReader};
use std::cmp;

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
        let val = val.parse::<i32>().unwrap();
        values.push(val);
    }

    let sum: i32 = (0..values.len()).fold(i32::MIN, |acc, i|
        {
            let mut m = 0;
            values[i..].iter().map(|x| {
                m += x;
                cmp::max(acc, m)
            })
            //.inspect(|x| println!("here {:?}", x) )
            .max().unwrap()
        });
    println!("{}", sum);
}
