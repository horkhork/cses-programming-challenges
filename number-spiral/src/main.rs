use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let mut line = "".to_string();
        input.read_line(&mut line).unwrap();

        let line: Vec<&str> = line.split_whitespace().collect();
        let y = line[0].parse::<i64>().unwrap();
        let x = line[1].parse::<i64>().unwrap();
        //println!("{} {}", x, y);

        let m = cmp::max(x, y);
        let d = (m * (m - 1)) + 1;
        
        let o = if m % 2 == 0 {
            d - (x - y)
        } else {
            d + (x - y)
        };
        //println!("{} {} Max {} diagonal {} o {}", y, x, m, d, o);
        println!("{}", o);

    };
}
