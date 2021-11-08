use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();

    // Count of numbers in the list
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();

    //println!("{}", n);

    // In order to make indexing easy, start from zero, but set the zeroth item to None
    let mut r: Vec<Option<i32>> = (0..=n).map(|x| match x {
        0 => None,
        _ => Some(x)
    }).collect();

    //println!("{:?}", r);
    // Process the list of items and set its corresponding entry in r to None
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    for i in line.split_whitespace() {
        //println!("{} {:?}", i, r);
        r[i.parse::<usize>().unwrap()] = None;
    };

    // Look for remaining numbers in the list that haven't been None'd yet
    //println!("{:?}", r);
    let rem = r.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<_>>();
    //println!("{:?}", rem);
    match rem.len() {
        0 => eprintln!("None left!"),
        1 => println!("{}", rem[0]),
        _ => eprintln!("{:?}", rem),
    }
}
