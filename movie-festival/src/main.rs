use std::cmp;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();

    // map movie end times to movie start times so we can quickly iterate over
    // the movies by movie end time first end to last
    let mut movie_times: BTreeMap<i32, i32> = BTreeMap::new();

    let mut line = "".to_string();
    while let Ok(x) = input.read_line(&mut line) {
        if x == 0 {
            // End of the file
            break;
        }
        let mut split = line.split_whitespace();
        //println!("Line {} X:{} {:?}", line, x, split);

        let start = split.next().unwrap().parse().unwrap();
        let end = split.next().unwrap().parse().unwrap();

        let entry = movie_times.entry(end).or_insert(start);
        *entry = cmp::max(*entry, start);

        line = "".to_string();
    }
    //println!("Movies: {:?}", movie_times);

    let mut movie_cnt = 1;
    let mut movie_times = movie_times.iter();
    let mut prev = movie_times.next().unwrap();
    //println!("Prev:{:?}", prev);

    for next in movie_times {
        // if the previous end time (tuple index 0) is less than or equal to next
        // movie start time (tuple index 1) then increment the counter and update
        // previous to the next one
        if prev.0 <= next.1 {
            //println!("Found one");
            movie_cnt += 1;
            prev = next;
        }
        //println!("Next:{:?}", next);
    }

    println!("{}", movie_cnt);
}
