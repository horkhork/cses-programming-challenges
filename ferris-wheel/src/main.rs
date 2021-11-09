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
    let mut children = children.map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    children.sort_by(|a, b| a.partial_cmp(b).unwrap());


    let mut gondolas = 0;
    while children.len() > 0 {
        gondolas += 1;
        let small = children.remove(0);
        if children.len() == 0 {
            break;
        }
        let mut i = children.len() - 1;
        let _it = loop {
            if i < 0 {
                break None;
            };

            if small + children[i] <= x {
                let ret = Some(children[i]);
                children.remove(i);
                break ret;
            }

            //println!("small {} i {} {}", small, i, children[i]);
            if i == 0 {
                break None;
            };
            i -= 1;
        };
        //println!("{} it {:?}", gondolas, _it);
    }

    ////println!("{:?}", children);
    //let mut weight = 0;
    //let mut num_riders = 0;
    //while children.len() > 0 {
    //    //println!("here");
    //    if (weight + children[children.len() - 1]) <= x && num_riders < 2 {
    //        //println!("in");
    //        let child_weight = children.pop().unwrap();
    //        weight += child_weight;
    //        if num_riders == 0 {
    //            //println!("new gondola {}", child_weight);
    //            gondolas += 1;
    //        } else {
    //            //println!("add to gondola {}", child_weight);
    //        }
    //        num_riders += 1;
    //    } else {
    //        weight = 0;
    //        num_riders = 0;
    //        //println!("not {}", gondolas);
    //    };
    //}

    println!("{}", gondolas);
}
