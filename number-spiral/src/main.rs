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
        //println!("{} {} diagonal {}", y, x, d);
        
        let o = if x > y {
            d + (x - y)
        } else {
            d + (y - x)
        };
        //println!("o {}", o);
        println!("{}", o);

    };
}
