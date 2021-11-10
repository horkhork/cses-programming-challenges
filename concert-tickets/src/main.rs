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
    //println!("tickets {:?}", tickets);

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let customers = line.split_whitespace();
    let customers = customers.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    //println!("customers {:?}", customers);

    for c in customers {
        //println!("Customer max {}", c);

        let mut val = -1;
        let mut prev = 0;
        let mut i = 0;
        let val = loop {
            if i >= tickets.len() {
                tickets[prev] = None;
                //println!("fell off the end");
                break val;
            }

            if tickets[i].is_none() {
                i += 1;
                continue;
            }

            // Current ticket price
            let t = tickets[i].unwrap();
            //println!("curr tik {}", t);

            if t < c {
                // Keep track of the previous ticket price customer would buy
                prev = i;
                val = t;
                //println!("found one {}", t);
            } else if t == c {
                tickets[i] = None;
                //println!("found it {}", t);
                break t;
            } else {
                tickets[prev] = None;
                //println!("break with prev {}", val);
                break val;
            }

            i += 1;
        };
        println!("{}", val);
    }
}

