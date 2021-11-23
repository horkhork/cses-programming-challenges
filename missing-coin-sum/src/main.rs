#![feature(control_flow_enum)]
use std::io::{BufRead, BufReader};
use std::ops::ControlFlow;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    //let mut split = line.split_whitespace();
    //let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();

    let mut values: Vec<i64> = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();
    values.sort_unstable();
    let missing = values.iter().try_fold(1_i64, |acc, x| {
        if x <= &acc {
            ControlFlow::Continue(acc + x)
        } else {
            ControlFlow::Break(acc)
        }
    });
    let missing = match missing {
    ControlFlow::Continue(x) => x,
    ControlFlow::Break(x) => x,
    };

    println!("{}", missing);


    //// Algorithm replicated from https://www.geeksforgeeks.org/find-smallest-value-represented-sum-subset-given-array/
    //let mut val: Option<i64> = None;
    //let mut result = 1;
    //for i in 0..values.len() {
    //    //println!("i:{} values:{} result:{}", i, values[i], result);
    //    if values[i] <= result {
    //        result += values[i];
    //    } else {
    //        //println!("missing {}", result);
    //        //println!("{}", result);
    //        val = Some(result);
    //        break;
    //    }
    //}
    //if val.is_none() {
    //    val = Some(values.iter().sum::<i64>() + 1);
    //}
    //println!("{}", val.unwrap());

    //let values: BTreeMap<i64,usize> = line
    //    .split_whitespace()
    //    .filter_map(|v| v.parse::<i64>().ok())
    //    .fold(BTreeMap::new(), |mut acc, v| {
    //        *acc.entry(v).or_insert(0) += 1;
    //        acc
    //        });
    //println!("{:?}", values);
    //let mut result = 1;
    //for i in 0..values.len() {
    //    let i = &(i as i64);
    //    if values[i] <= result {
    //        result += values[i]
    //    }
    //}

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
