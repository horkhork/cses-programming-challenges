use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    //let mut split = line.split_whitespace();
    //let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();

    let values: Vec<i64> = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();
    let v2 = values.clone();

    //let values = values
    //    .iter()
    //    .flat_map(|x| v2[1..].iter().map(|y| vec![x, y]).collect::<Vec<Vec<&i64>>>())
    //    .inspect(|v| println!("V{:?}", v))
    //    .collect::<Vec<Vec<&i64>>>();
    //println!("Values: {:?}", values);

    let values = values
        .iter()
        .flat_map(|x| {
            v2[1..]
                .iter()
                .map(|y| vec![x, y])
                .collect::<Vec<Vec<&i64>>>()
        })
        .inspect(|v| println!("V{:?}", v))
        .fold(BTreeSet::new(), |mut acc: BTreeSet<i64>, mut val| {
            acc.insert(val.iter().map(|n| *n).sum::<i64>());
            acc
        });
    println!("Values: {:?}", values);

    ////let values: Vec<i64> = line.split_whitespace().filter_map(|v| v.parse::<i64>().ok()).collect();
    //let best = line.split_whitespace().filter_map(|v| v.parse::<i64>().ok())
    //    //.iter()
    //    .fold((i64::MIN, 0), |(mut best, mut current), x| {
    //        if current <= 0 {
    //            current = x;
    //        } else {
    //            current += x;
    //        }
    //        if current > best {
    //            best = current;
    //        }
    //        (best, current)
    //    });
    //println!("{:?}", best.0);
}