use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

// 5 3
//        4 2 1 5 3 -> 3
// 2 3 -> 4 1 2 5 3 -> 2
// 1 5 -> 3 1 2 5 4 -> 3
// 2 3 -> 3 2 1 5 4 -> 4

// Hypothetical cases
//  One Change
//        4 2 1 5 3 -> 3
//        _   _
// 1 3 -> 1 2 4 5 3 -> 2
//
//  One Change
//        4 2 1 5 3 -> 3
//        _     _
// 1 4 -> 5 2 1 4 3 -> 4
//
//  One Change
//        4 2 1 5 3 -> 3
//          _   _
// 2 4 -> 4 5 1 2 3 -> 2
//
//  No Change
//        4 2 1 5 3 -> 3
//              _ _
// 4 5 -> 4 2 1 3 5 -> 3
//
//  No Change
//            _ _
//        4 2 1 5 3 -> 3
// 3 4 -> 4 2 5 1 3 -> 3
//
fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();

    let (n, _m) = match lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<i32>>()[..]
    {
        [n, m] => (n, m),
        _ => panic!("First line not valid"),
    };

    let mut arr: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i64>().ok())
        .collect();
    //println!("Arr:{:?}", arr);

    let mut indexes: BTreeMap<i64, usize> = BTreeMap::new();
    let mut round: usize = arr.iter().enumerate()
        .map(|(i, x)| {
            indexes.insert(*x, i);
            match indexes.contains_key(&(x - 1)) {
                true => 0,
                false => 1,
            }
        })
        .sum();
    println!("Arr:{:?}", arr);
    println!("Indexes:{:?}", indexes);
    println!("Initial Count:{:?}", round);

    for line in lines {
        let (a, b) = match line
            .unwrap()
            .split_whitespace()
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<Vec<usize>>()[..]
        {
            [a, b] => match a < b {
                true => (a - 1, b - 1),
                false => (b - 1, a - 1),
            },
            _ => panic!("line not valid"),
        };
        let orig_a = arr[a];
        let orig_b = arr[b];
        arr.swap(a, b);
        let new_a = arr[a];
        let new_b = arr[b];
        println!("A:{} B:{} OrigA:{} OrigB:{} NewA:{} NewB:{}", a, b, orig_a, orig_b, new_a, new_b);
        //if a > 0 && orig_b > 1 {
        //    if indexes[&(orig_b - 1)] > a && indexes[&(orig_b - 1)] <= b {
        //        println!("A:Minus 1 {} > {} && {} <= {}", indexes[&(orig_b - 1)], a, indexes[&(orig_b - 1)], b);
        //        round -= 1;
        //} else if b < (n as i64 - 1) as usize && orig_a < n as i64 {
        //    if indexes[&(orig_a - 1)] > a && indexes[&(orig_a - 1)] <= b {
        //        println!("B:Minus 1 {} > {} && {} <= {}", indexes[&(orig_a - 1)], a, indexes[&(orig_a - 1)], b);
        //        round -= 1;
        //    }
        //    }
        //}
        //// Edge case
        //if ((a - b) as i32).abs() == 1 && new_a == new_b - 1{
        //    round -= 1;
        //} else if ((a - b) as i32).abs() == 1 && new_a -1 == new_b {
        //    round += 1;
        //} else {
        //    println!("else");
        //}
        ////if a > 0 {
        ////    if
        println!("Arr:{:?} === {}", arr, round);

        indexes.insert(new_a, a);
        indexes.insert(new_b, b);
        println!("Indexes {:?}", indexes);
    }
}

//fn collect_numbers(arr: &[i64]) -> usize {
//    let mut set: BTreeSet<i64> = BTreeSet::new();
//    arr.iter()
//        .map(|x| {
//            set.insert(*x);
//            match set.contains(&(x - 1)) {
//                true => 0,
//                false => 1,
//            }
//        })
//        .sum()
//}
