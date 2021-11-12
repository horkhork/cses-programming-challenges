use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut start_times: BTreeMap<i32, i32> = BTreeMap::new();
    let mut end_times: BTreeMap<i32, i32> = BTreeMap::new();

    let mut line = "".to_string();
    while let Ok(x) = input.read_line(&mut line) {
        if x == 0 {
            // End of the file
            break;
        }
        let mut split = line.split_whitespace();
        //println!("Line {} X:{} {:?}", line, x, split);
        *start_times
            .entry(split.next().unwrap().parse().unwrap())
            .or_insert(0) += 1;
        *end_times
            .entry(split.next().unwrap().parse().unwrap())
            .or_insert(0) += 1;
        line = "".to_string();
    }
    //println!("Starts: {:?}", start_times);
    //println!("Ends: {:?}", end_times);

    let mut curr_cnt = 0;
    let mut movie_cnt = 0;
    let mut last_key = 0;
    for (&key, val) in start_times.iter() {
        //println!("Looking for ends {}..{}", last_key, key);
        for (_k, v) in end_times.range(last_key+1..=key) {
            curr_cnt -= v;
            //println!("End {} with {} curr {}", _k, v, curr_cnt);
        }

        curr_cnt += val;
        //println!("Start {} with {} curr {}", key, val, curr_cnt);
        if curr_cnt == 1 {
            //println!("watching movie starting at {}", key);
            movie_cnt += 1;
        }
        last_key = key;
    }

    println!("{}", movie_cnt);
}
