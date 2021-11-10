use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

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
    let tickets = tickets.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut ticket_prices = HashSet::new();

    let mut tik_map = HashMap::new();
    for tik in tickets.iter() {
        //println!("tik {}", tik);
        let cnt = tik_map.entry(tik).or_insert(0);
        *cnt += 1;
        ticket_prices.insert(*tik);
    }
    let mut ticket_prices: Vec<&i32> = ticket_prices.iter().collect();
    ticket_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //println!("tickets_prices {:?}", ticket_prices);
    //println!("tickets {:?}", tik_map);
    let len = ticket_prices.len();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let customers = line.split_whitespace();
    let customers = customers.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    //println!("customers {:?}", customers);

    for c in customers {
        //println!("Customer max {}", c);

        //println!("tickets {:?}", tik_map);
        let mut val = &-1;
        //for t in &ticket_prices {

        let mut i = 0;
        loop {
            if i == len {
                //println!("fell off the end");
                tik_map.entry(&val).and_modify(|x| { *x -= 1 });
                break;
            }
            let t = &ticket_prices[i];
            i += 1;
            if tik_map.get(t).unwrap() <= &0 {
                //println!("Zero left for {}", t);
                continue;
            }
            // Ticket price is less than customer max, record the value but go on
            if **t < c {
                // Keep track of the previous ticket price customer would buy
                val = *t;
                //println!("found one {}", t);
            // Ticket price is exact match of customer max, return it
            } else if **t == c {
                val = *t;
                tik_map.entry(&val).and_modify(|x| { *x -= 1 });
                //println!("found it {}", t);
                break;
            // Ticket price exceeds customer match, return previous
            } else {
                tik_map.entry(&val).and_modify(|x| { *x -= 1 });
                //println!("break with prev {}", val);
                break;
            }
        }
        println!("{}", val);
    }
}

