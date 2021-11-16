//use std::collections::{BTreeMap, BTreeSet};
//use std::fmt;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    //let mut split = line.split_whitespace();
    //let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut values = Vec::new();
    for val in line.split_whitespace() {
        let val = val.parse::<i32>().unwrap();
        values.push(val);
    }
    //println!("values {:?}", values);
    let len = values.len();
    let mut d = Diff { d: None };
    for i in 1..=len {
        d.combinate_vec(vec![], values.clone(), i, values.clone());
    }
    println!("{}", d.d.unwrap());
}

struct Diff {
    d: Option<u32>,
}

impl Diff {
    fn combinate_vec(&mut self, accum: Vec<i32>, rest: Vec<i32>, n: usize, all: Vec<i32>) {
        if n == 0 {
            let rest: Vec<_> = all.into_iter().filter(|x| !accum.contains(x)).collect();
            let diff: i32 = accum.iter().sum::<i32>() - rest.iter().sum::<i32>();
            let diff: u32 = diff.abs() as u32;
            if self.d.is_none() || self.d.unwrap() > diff {
                self.d = Some(diff);
            }

            //println!("Done Accum:{:?} Rest:{:?} diff:{}", accum, rest, diff);
        } else {
            for i in 0..rest.len() {
                let mut a = accum.clone();
                a.push(rest[i]);
                self.combinate_vec(a, rest[i + 1..].to_vec(), (n - 1) as usize, all.clone());
            }
        }
    }
}
