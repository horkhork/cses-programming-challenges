use std::collections::{BTreeSet, BTreeMap};
use std::io::{BufRead, BufReader};
use std::fmt;

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
        values.push(val.parse::<i32>().unwrap());
    }
    println!("values {:?}", values);

    let apples = Apples{
        group1: BTreeMap::from([(values.iter().sum(), values)]),
        group2: BTreeMap::from([(0, vec![])]),
    };

    println!("apples {}", apples);
    //subset_sums(values.into_iter().collect(), sum / 2, vec![]);

}

struct Apples {
    // Maps the sum to the component weights
    group1: BTreeMap<i32, Vec<i32>>,
    group2: BTreeMap<i32, Vec<i32>>,
}

impl fmt::Display for Apples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.group1, self.group2)
    }
}

//fn subset_sums(all_numbers: Parts, numbers: Vec<i32>, threshold: i32, part: Vec<i32>) {
//    let s: i32 = part.iter().sum();
//    let mut counts: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
//    if numbers.len() == 0 {
//        counts.insert(part.iter().sum(), part);
//        return ;
//    //if s == threshold {
//    //    println!("EQ threshold {} with {}", threshold, s);
//    //    counts.insert(0, part);
//    //    return counts;
//    //} else if s > threshold {
//    //    println!("Exceeded threshold {} with {}", threshold, s);
//    //    let over = s - threshold;
//    //    counts.insert(over, part);
//    //    return counts;
//    //}
//    }
//
//    for (i, n) in numbers.iter().enumerate() {
//        let mut rem = Vec::new();
//        rem.extend_from_slice(&numbers[i+1..]);
//        let mut p = Vec::new();
//        p.extend_from_slice(&part);
//        p.push(*n);
//        let mut ret = subset_sums(all_numbers, rem, threshold, p);
//
//        //println!("list {} {} {:?}+{}", (part.iter().sum::<i32>() + n)-threshold, threshold, part, n);
//    }
//
//}
