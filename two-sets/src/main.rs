use std::io::{BufRead, BufReader};
//use std::cmp;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n: i64 = split.next().unwrap().parse().unwrap();

    let sum = permute(n);
    if sum % 2 != 0 {
        println!("NO");
        return;
    }

    let half: i64 = sum / 2;

    let all_nums: Vec<i64> = (0..=n).collect();
    //println!("{:?}", all_nums);
    let mut first_set: Vec<i64> = Vec::new();
    let mut sum = 0;
    for num in all_nums.iter().rev() {
        if sum + num <= half {
            sum += num;
            first_set.push(*num);
        };
        if sum == half {
            //println!("found it {} {:?}", sum, first_set);
            break;
        }
    }
    if sum != half || first_set.len() == 0 {
        println!("NO");
        return;
    }

    println!("YES");

    println!("{}", first_set.len());
    println!(
        "{}",
        first_set
            .iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    let mut all_nums: Vec<Option<i64>> = all_nums
        .iter()
        .map(|x| if x > &0 { Some(*x) } else { None })
        .collect();
    for i in first_set {
        //println!("remove {}", i.to_string());
        all_nums[i as usize] = None;
    }
    let rest: Vec<i64> = all_nums
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    println!("{}", rest.len());
    println!(
        "{}",
        rest.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn permute(n: i64) -> i64 {
    let mut ret = 0;
    for i in 0..=n {
        ret += i;
    }
    ret
}
