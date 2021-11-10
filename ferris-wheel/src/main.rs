use std::io::{BufRead, BufReader};
//use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();
    let x: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let children = line.split_whitespace();
    let mut children = children.map(|x| Some(x.parse::<i32>().unwrap())).collect::<Vec<Option<i32>>>();
    children.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //println!("children {:?}", children);

    let len = children.len();
    let mut gondolas = 0;
    let mut i = 0; // Index from the head of the Vec
    let mut j = len-1; // Index from the end of the Vec

    loop {
        if i >= len {
            break;
        }
        let mut gondola_weight = 0;
        if children[i].is_none() {
            i += 1;
            continue;
        }
        gondola_weight += children[i].unwrap();
        children[i] = None;
        gondolas += 1;

        loop {
            if children[j].is_some() && gondola_weight + children[j].unwrap() <= x {
                children[j] = None;
                break;
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }

        i += 1;
    }

    println!("{}", gondolas);
}
