use std::io::{BufRead, BufReader};
//use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _: i64 = split.next().unwrap().parse().unwrap();
    let x: i64 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let children = line.split_whitespace();
    let mut children = children.map(|x| Some(x.parse::<i64>().unwrap())).collect::<Vec<Option<i64>>>();
    children.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //println!("children {:?}", children);

    let len = children.len();
    let mut gondolas = 0;
    for i in 0..len {
        if children[i].is_none() {
            continue;
        }
        gondolas += 1;
        let curr = children[i].unwrap();
        children[i] = None;
        //println!("curr {}", curr);
        for j in (i..len).rev() {
            if children[j].is_none() {
                continue;
            }
            //println!("curr {} new {}", curr, children[j].unwrap());
            if curr + children[j].unwrap() <= x {
                children[j] = None;
                break;
            }
        }
    }

    println!("{}", gondolas);
}
