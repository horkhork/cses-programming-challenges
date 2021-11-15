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
    //println!("{} values: {:?}", sum, values);

    let ret = subset_sums(values.into_iter().collect(), sum / 2, vec![]);
    let lowest = ret.iter().next().unwrap();
    println!("{}", lowest.0);
    println!("{:?}", ret);
}

fn subset_sums(numbers: Vec<i32>, threshold: i32, part: Vec<i32>) -> BTreeMap<i32, Vec<i32>> {
    let s: i32 = part.iter().sum();
    let mut counts: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    if s == threshold {
        println!("EQ threshold {} with {}", threshold, s);
        counts.insert(0, part);
        return counts;
    } else if s > threshold {
        println!("Exceeded threshold {} with {}", threshold, s);
        let over = s - threshold;
        counts.insert(over, part);
        return counts;
    }

    for (i, n) in numbers.iter().enumerate() {
        let mut rem = Vec::new();
        rem.extend_from_slice(&numbers[i+1..]);
        let mut p = Vec::new();
        p.extend_from_slice(&part);
        p.push(*n);
        let mut ret = subset_sums(rem, threshold, p);
        counts.append(&mut ret);

        //println!("list {} {} {:?}+{}", (part.iter().sum::<i32>() + n)-threshold, threshold, part, n);
    }

    return counts;
}
