use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let x: i32 = split.next().unwrap().parse().unwrap();

    let mut values: BTreeSet<i32> = BTreeSet::new();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for val in line.split_whitespace() {
        values.insert(val.parse::<i32>().unwrap());
    }
    println!("n:{} ={} values: {:?}", n, x, values);
    let second_half = values.split_off(&(x/2));
    let mut second_half: Vec<&i32> = second_half.iter().collect();
    second_half.reverse();
    for (i, a) in values.iter().enumerate() {
        //println!("check: {}", a);
        for (j, b) in second_half.iter().enumerate() {
            println!("check: a:{}({}) b:{}({})", a, i, b, j);
            if a + *b == x {
                println!("Found one: a:{}({}) b:{}({})", a, i, b, j);
            }
        }
    }

    println!("{}", -1);
}
