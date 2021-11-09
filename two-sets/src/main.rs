use std::io::{BufRead, BufReader};
//use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i64 = split.next().unwrap().parse().unwrap();

    let sum = permute(n);
    if sum % 2 != 0 {
        println!("NO");
        return;
    }

    let half = sum / 2;
    println!("{}", half);

    let mut nums: Vec<i64> = (0..=n).collect();
    nums.reverse();
    let mut sum = 0;
    println!("{:?}", nums);
    let mut tally: Vec<i64> = Vec::new();
    for num in nums {
        if sum + num <= half {
            sum += num;
            tally.push(num);
        };
        if sum == half {
            println!("found it {} {:?}", sum, tally);
            return;
        }
    };
    println!("NO");
}

fn permute(n: i64) -> i64 {
    let mut ret = 0;
    for i in 0..=n {
        ret += i;
    }
    ret
}
