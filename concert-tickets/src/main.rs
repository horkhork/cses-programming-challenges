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

    // HashSet of unique ticket prices
    let mut ticket_prices = HashSet::new();

    // Keep track of ticket prices and the number of times each price occurs
    let mut tik_map = HashMap::new();
    for tik in line.split_whitespace() {
        let tik = tik.parse::<i32>().unwrap();
        //println!("tik {}", tik);
        let cnt = tik_map.entry(tik).or_insert(0);
        *cnt += 1;
        ticket_prices.insert(tik);
    }
    let mut ticket_prices: Vec<&i32> = ticket_prices.iter().collect();
    ticket_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //println!("tickets_prices {:?}", ticket_prices);
    //println!("tickets {:?}", tik_map);
    let len = ticket_prices.len();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    //println!("customers {:?}", customers);

    for customer_max in line.split_whitespace() {
        let customer_max = customer_max.parse::<i32>().unwrap();
        //println!("Customer max {}", customer_max);

        //println!("tickets {:?}", tik_map);
        let mut curr_price_to_pay = -1;
        let mut idx = 0;
        loop {
            if idx == len {
                //println!("fell off the end");
                tik_map.entry(curr_price_to_pay).and_modify(|x| { *x -= 1 });
                break;
            }
            let curr_tik_price = &ticket_prices[idx];
            if tik_map.get(curr_tik_price).unwrap() <= &0 {
                //println!("Zero left for {}", curr_tik_price);
                continue;
            }
            idx += 1;
            
            // Ticket price is exact match of customer max, return it
            if **curr_tik_price == customer_max {
                curr_price_to_pay = **curr_tik_price;
                tik_map.entry(curr_price_to_pay).and_modify(|x| { *x -= 1 });
                //println!("found it {}", curr_tik_price);
                break;
            }

            // Ticket price is less than customer max, record the value but go on
            if **curr_tik_price < customer_max {
                // Keep track of the previous ticket price customer would buy
                curr_price_to_pay = **curr_tik_price;
                //println!("found one {}", curr_tik_price);

            // Ticket price exceeds customer match, return previous
            } else {
                tik_map.entry(curr_price_to_pay).and_modify(|x| { *x -= 1 });
                //println!("break with prev {}", curr_price_to_pay);
                break;
            }
        }
        println!("{}", curr_price_to_pay);
    }
}

