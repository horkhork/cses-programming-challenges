use std::collections::{BTreeSet, BTreeMap};
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    //let mut split = line.split_whitespace();
    //let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut sum = 0;
    let mut values: BTreeSet<i32> = BTreeSet::new();
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for val in line.split_whitespace() {
        let val = val.parse::<i32>().unwrap();
        values.insert(val);
        sum += val;
    }
    let sum1 = sum / 2;
    let sum2 = sum - sum1;
    println!("Sums {} {}", sum1, sum2);

    subset_sums(values.into_iter().collect(), sum / 2, vec![]);
}

struct Parts {
    part1: BTreeMap<i32, Vec<i32>>,
    part2: BTreeMap<i32, Vec<i32>>,
}

fn subset_sums(all_numbers: Parts, numbers: Vec<i32>, threshold: i32, part: Vec<i32>) {
    let s: i32 = part.iter().sum();
    let mut counts: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    if numbers.len() == 0 {
        counts.insert(part.iter().sum(), part);
        return ;
    //if s == threshold {
    //    println!("EQ threshold {} with {}", threshold, s);
    //    counts.insert(0, part);
    //    return counts;
    //} else if s > threshold {
    //    println!("Exceeded threshold {} with {}", threshold, s);
    //    let over = s - threshold;
    //    counts.insert(over, part);
    //    return counts;
    //}
    }

    for (i, n) in numbers.iter().enumerate() {
        let mut rem = Vec::new();
        rem.extend_from_slice(&numbers[i+1..]);
        let mut p = Vec::new();
        p.extend_from_slice(&part);
        p.push(*n);
        let mut ret = subset_sums(all_numbers, rem, threshold, p);

        //println!("list {} {} {:?}+{}", (part.iter().sum::<i32>() + n)-threshold, threshold, part, n);
    }

}
