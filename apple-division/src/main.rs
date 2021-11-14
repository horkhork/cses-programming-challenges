use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut sum = 0;
    let mut values: BTreeSet<i32> = BTreeSet::new();
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for val in line.split_whitespace() {
        let val = val.parse::<i32>().unwrap();
        values.insert(val);
        sum += val;
    }
    //println!("{} values: {:?}", sum, values);
    let part1 = sum / 2;
    let part2 = sum - part1;
    //println!("{} {}", part1, part2);
    //
    println!("{}", part2- part1);

    //for (a, i) in values.iter() {
    //    //println!("check: {}", a);
    //    if let Some(j) = second_half.get(&(x - a)) {
    //        println!("{} {}", i + 1, j + 1);
    //        return;
    //    };
    //}

}
