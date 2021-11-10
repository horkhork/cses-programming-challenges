use std::io::{BufRead, BufReader};
//use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    let _m: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let tickets = line.split_whitespace();
    let mut tickets = tickets.map(|x| Some(x.parse::<i32>().unwrap())).collect::<Vec<Option<i32>>>();
    tickets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("tickets {:?}", tickets);

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let customers = line.split_whitespace();
    let customers = customers.map(|x| Some(x.parse::<i32>().unwrap())).collect::<Vec<Option<i32>>>();
    println!("customers {:?}", customers);

    let mut i = 0;
    let mut j = 0;
    let mut highwater = 0;
    loop {
        if j >= customers.len() {
            break;
        }
        loop {
            if i >= tickets.len() {
                println!("-1");
                break;
            }
            if customers[j] >= tickets[i] {
                highwater = tickets[i].unwrap();
                println!("{}", highwater);
            i += 1;
                break;
            }
            i += 1;
        }
        j += 1;
    }
}

