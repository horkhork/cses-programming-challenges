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
    println!("{} values: {:?}", sum, values);
    let part1 = sum / 2;
    let part2 = sum - part1;
    println!("{} {}", part1, part2);
    //
    println!("{}", part2- part1);

    let mut s = -1;
    //let to_remove = 0;
    for val in values.iter().rev() {
        if s + val <= part2 {
            s += val;
            println!("add {}; {}", val, s);
        } else {
            println!("nope {}", val);
        }
        //to_remove =
    }
    let ret = subset_sums(values.into_iter().collect(), sum / 2, vec![]);
    //for (a, i) in values.iter() {
    //    //println!("check: {}", a);
    //    if let Some(j) = second_half.get(&(x - a)) {
    //        println!("{} {}", i + 1, j + 1);
    //        return;
    //    };
    //}

}


enum Possibilities {
    Lt,
    Eq,
    Gt,
}

fn subset_sums(numbers: Vec<i32>, threshold: i32, part: Vec<i32>) -> Possibilities {
    let s: i32 = part.iter().sum();
    if s == threshold {
        println!("EQ threshold {} with {}", threshold, s);
        return Possibilities::Eq;
    } else if s > threshold {
        //println!("Exceeded threshold {} with {}", threshold, s);
        return Possibilities::Gt;
    }

    for (i, n) in numbers.iter().enumerate() {
        let mut rem = Vec::new();
        rem.extend_from_slice(&numbers[i+1..]);
        let mut p = Vec::new();
        p.extend_from_slice(&part);
        p.push(*n);
        subset_sums(rem, threshold, p);
        println!("list {} {} {:?}+{}", (part.iter().sum::<i32>() + n)-threshold, threshold, part, n);
    }

    return Possibilities::Lt;
}
