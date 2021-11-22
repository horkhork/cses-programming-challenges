use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut values = Vec::new();
    for val in line.split_whitespace() {
        let val = val.parse::<i32>().unwrap();
        values.push(val);
    }

    //let sum: i32 = (0..values.len()).fold(i32::MIN, |acc, i|
    //    {
    //        let mut m = 0;
    //        values[i..].iter().map(|x| {
    //            m += x;
    //            cmp::max(acc, m)
    //        })
    //        //.inspect(|x| println!("here {:?}", x) )
    //        .max().unwrap()
    //    });

    //let negs: Vec<i32> = values.filter(|x| x <
    let mut negative_indexes: Vec<usize> = values.iter().enumerate().filter(|(_, &x)| x < 0).map(|(i, _)| i).collect();
    if negative_indexes[0] != 0 {
        negative_indexes.insert(0,0);
    }
    let len = negative_indexes.len();
    let val_len = values.len();
    if negative_indexes[len - 1] != val_len - 1 {
        negative_indexes.push(val_len - 1);
    }
    println!("{:?}", negative_indexes);
    let negative_indexes2 = negative_indexes.clone();
    //let it: Vec<(&usize,&usize)> = negative_indexes.iter().zip(negative_indexes_cp.iter()).filter(|(x,y)| x == y).collect();
    //let it: Vec<(&usize,&usize)> = negative_indexes.iter().flat_map(|x| negative_indexes.clone().iter().map(|y| (x,y))).collect();
    for x in negative_indexes {
        for y in &negative_indexes2 {
            if x == *y {
                continue;
            }
            println!("{} {}", x,y);
        }
    }

}
