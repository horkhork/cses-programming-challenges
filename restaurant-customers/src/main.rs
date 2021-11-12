use std::cmp;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    //let _m: i32 = split.next().unwrap().parse().unwrap();
    //let k: i32 = split.next().unwrap().parse().unwrap();

    let mut start_times = BTreeMap::new();
    let mut end_times = BTreeMap::new();

    // Store desired apartment size
    let mut line = "".to_string();
    while let Ok(x) = input.read_line(&mut line) {
        if x == 0 {
            // End of the file
            break;
        }
        let mut split = line.split_whitespace();
        //println!("Line {} X:{} {:?}", line, x, split);
        let s: i32 = split.next().unwrap().parse().unwrap();
        let e: i32 = split.next().unwrap().parse().unwrap();
        let p = start_times.entry(s).or_insert(0);
        *p += 1;
        let p = end_times.entry(e).or_insert(0);
        *p += 1;
        line = "".to_string();
    }
    //println!("Starts: {:?}", start_times);
    //println!("Ends: {:?}", end_times);

    let mut curr_cnt = 0;
    let mut highwater_mark = 0;
    let mut last_key = 0;
    for (&key, val) in start_times.iter() {
        for (_, v) in end_times.range(last_key..=key) {
            //println!("End {} with {}", k, v);
            curr_cnt -= v;
        }

        //println!("Start {} with {}", key, val);
        curr_cnt += val;
        highwater_mark = cmp::max(curr_cnt, highwater_mark);
        last_key = key;
    }

    println!("{}", highwater_mark);
}
