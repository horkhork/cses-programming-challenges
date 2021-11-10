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
    loop {
        if j >= customers.len() {
            break;
        }
        // customers max price
        let m = customers[j].unwrap();
        println!("Customer max {}", m);

        loop {
            let mut val = -1;
            if i >= tickets.len() {
                println!("{}", val);
                break;
            }
            // Current ticket price
            let t = tickets[j].unwrap();
            println!("curr tik {}", t);

            if t <= m {
                // Keep track of the previous ticket price customer would buy
                val = t;
            }

            i += 1;
            //if customers[j] < tickets[i] {
            //    println!("LessEq {} {} {} {}", j, customers[j].unwrap(), i, tickets[i].unwrap());
            //    i += 1;
            //} else if customers[j] >= tickets[i] {
            //    println!("Found One {} {} {} {}", j, customers[j].unwrap(), i, tickets[i].unwrap());
            //    println!("{}", tickets[j].unwrap());
            //    i += 1;
            //    break;
            //} else {
            //    println!("More {} {} {} {}", j, customers[j].unwrap(), i, tickets[i].unwrap());
            //    if i == 0 {
            //        println!("At zero");
            //        break
            //    }
            //    i -= 1;
            //    println!("{}", tickets[i].unwrap());
            //    break;
            //}
        }

        j += 1;
    }
}

