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
}
