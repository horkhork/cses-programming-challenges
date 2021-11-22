use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut values = Vec::new();
    for val in line.split_whitespace() {
        let val = val.parse::<i64>().unwrap();
        values.push(val);
    }

    //println!("Values:{:?}", values);
    //println!("Mean:{:?}", mean(&values));
    //println!("Median:{:?}", median(&mut values));
    let med = median(&mut values);
    let sum = values.iter().fold(0, |acc, x| acc + (x - med).abs());
    //println!("Sum:{}", sum);
    println!("{}", sum);
}

fn mean(numbers: &Vec<i64>) -> f64 {

    let sum: i64 = numbers.iter().sum();

    sum as f64 / numbers.len() as f64

}

fn median(numbers: &mut Vec<i64>) -> i64 {

    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i64
    } else {
        numbers[mid]
    }

}
