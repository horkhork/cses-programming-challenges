use std::io::{BufRead, BufReader};
//use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let x: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let children = line.split_whitespace();
    let mut children = children.map(|x| Some(x.parse::<i32>().unwrap())).collect::<Vec<Option<i32>>>();
    children.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //println!("children {:?}", children);

    let len = children.len();
    let mut gondolas = 0;
    let mut i = 0;
    let mut j = len-1;
    let mut processed_children = 0;

    //println!("{} {}", children[i].unwrap(), children[j].unwrap());
    loop {
        if i >= len {
            //println!("fell off the end");
            break;
        }
        let mut gondola_weight = 0;
        if children[i].is_none() {
            //println!("child[i] is None");
            i += 1;
            continue;
        }
        if processed_children == n {
            println!("processed all children");
            break;
        }
        gondola_weight += children[i].unwrap();
        children[i] = None;
        processed_children += 1;
        gondolas += 1;

        loop {
            if children[j].is_some() {
                if gondola_weight + children[j].unwrap() <= x {
                    processed_children += 1;
                    children[j] = None;
                    break;
                }
            }
            if j == 0 {
                //println!("j is zero");
                break;
            }
            j -= 1;
        }

        i += 1;
    }

    //println!("children {:?}", children);

    //for i in 0..len {
    //    if children[i].is_none() {
    //        continue;
    //    }
    //    gondolas += 1;
    //    let curr = children[i].unwrap();
    //    children[i] = None;
    //    //println!("curr {}", curr);
    //    for j in (i..len).rev() {
    //        if children[j].is_none() {
    //            continue;
    //        }
    //        //println!("curr {} new {}", curr, children[j].unwrap());
    //        if curr + children[j].unwrap() <= x {
    //            children[j] = None;
    //            break;
    //        }
    //    }
    //}

    println!("{}", gondolas);
}
