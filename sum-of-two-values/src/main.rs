use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    let x: i32 = split.next().unwrap().parse().unwrap();

    let mut values: BTreeMap<i32, i32> = BTreeMap::new();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for (i, val) in line.split_whitespace().enumerate() {
        values.insert(val.parse::<i32>().unwrap(), i as i32);
        //println!("Adding {} at index {}", val, i);
    }
    //println!("n:{} ={} values: {:?}", n, x, values);
    let second_half = values.split_off(&(x / 2));
    //println!("First {:?}", values);
    //println!("Second {:?}", second_half);
    for (a, i) in values.iter() {
        //println!("check: {}", a);
        if let Some(j) = second_half.get(&(x - a)) {
            println!("{} {}", i + 1, j + 1);
            return;
        };
    }

    println!("IMPOSSIBLE");
}
