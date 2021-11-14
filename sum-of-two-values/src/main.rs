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
    let half = x / 2;
    let mut half_loc1 = None;
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for (i, val) in line.split_whitespace().enumerate() {
        let val = val.parse::<i32>().unwrap();
        if val == half && half_loc1.is_none() {
            half_loc1 = Some(i);
        } else if val == half {
            println!("{} {}", half_loc1.unwrap() + 1, i + 1);
            return;
        }
        values.insert(val, i as i32);
        //println!("Adding {} at index {}", val, i);
    }
    //println!("n:{} ={} values: {:?}", n, x, values);
    let second_half = values.split_off(&(half + 1));
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
