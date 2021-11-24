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

    let (N, _m) = match lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect::<Vec<i32>>()[..]
    {
        [N, m] => (N, m),
        _ => panic!("First line not valid"),
    };

    let mut arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    //println!("Arr:{:?}", arr);

    let mut indexes: BTreeMap<i32, usize> = BTreeMap::new();
    let mut round: usize = arr.iter().enumerate()
        .map(|(i, x)| {
            indexes.insert(*x, i);
            match indexes.contains_key(&(x - 1)) {
                true => 0,
                false => 1,
            }
        })
        .sum();
    println!("N:{}", N);
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
            // Always move indexes to zero-based and ensure a < b
            [a, b] => match a < b {
                true => (a - 1, b - 1),
                false => (b - 1, a - 1),
            },
            _ => panic!("line not valid"),
        };
        let orig_a = arr[a];
        let orig_b = arr[b];
        println!("\nProcessing {:?}::: IdxA:{} IdxB:{} ValA:{} ValB:{}", arr, a, b, orig_a, orig_b);
        arr.swap(a, b);
        let new_a = arr[a];
        let new_b = arr[b];
        if b - a == 1 {
            println!("adjacent swap");
            match orig_a < orig_b {
                true => round += 1,
                false => round -= 1,
            }
        } else {
            if (orig_a-orig_b).abs() == 1 {
                println!("monotonic neighbor swap");
                match orig_a < orig_b {
                    true => round += 1,
                    false => round -= 1,
                }
            }
            match orig_a {
                i if i == 1 => {
                    println!("Orig A == 1");
                    let one_more = i + 1;
                    let one_more_idx = indexes[&one_more];
                    println!("---Orig A == 1 val {} (,{}); Idx:{} < IdxB:{}", i, one_more, one_more_idx, b);
                    if one_more_idx > a && one_more_idx < b {
                        println!("AAA plus 1");
                        round += 1;
                    } else {
                        println!("AAA minus 1");
                        round -= 1;
                    }
                },
                i if i == N => {
                    println!("Orig A end val {}", i);
                },
                i => {
                    let one_less = i - 1;
                    let one_less_idx = indexes[&one_less];
                    let one_more = i + 1;
                    let one_more_idx = indexes[&one_more];
                    println!("Orig A middle val {} ({},{}); -idx:{} <= a:{} && +idx:{} < b:{}", i, one_less, one_more, one_less_idx, a, one_more_idx, b);
                    if one_less_idx <= a && one_more_idx < b {
                        println!("BBB minus 1");
                        round -= 1;
                    } else {
                        println!("BBB plus 1");
                        round += 1;
                    }
                },
            };
            match orig_b {
                i if i == 1 => {
                    println!("Orig B == 1");
                    let one_more = i + 1;
                    let one_more_idx = indexes[&one_more];
                    println!("---Orig B middle val {} (,{}); {} > {}", i, one_more, one_more_idx, a);
                    if one_more_idx > a {
                        println!("CCC minus 1");
                        round -= 1;
                    } else {
                        println!("CCC plus 1");
                        round += 1;
                    }
                },
                i if i == N => {
                    println!("Orig B end val {}", i);
                },
                i => {
                    let one_less = i - 1;
                    let one_less_idx = indexes[&one_less];
                    let one_more = i + 1;
                    let one_more_idx = indexes[&one_more];
                    println!("Orig B middle val {} ({},{}); -idx:{} <= a:{} && +idx:{} < b:{}", i, one_less, one_more, one_less_idx, a, one_more_idx, b);
                    if one_less_idx <= a && one_more_idx < b {
                        println!("DDD minus 1");
                        round -= 1;
                    } else {
                        println!("DDD plus 1");
                        round += 1;
                    }
                },
            };
        }
        //let one_bigger_idx = indexes.get(&(orig_b + 1));
        //let one_less_idx = indexes.get(&(orig_a - 1));
        //// Add a round, we've moved a B value before its preceding N
        //if orig_b > 1 && indexes[&(orig_b - 1)] > a {
        //    round += 1;
        //}
        //if one_bigger_idx.is_some() && one_bigger_idx.unwrap() < &b && one_bigger_idx.unwrap() >= &a {
        //    println!("AAA {}", one_bigger_idx.unwrap());
        //    round -= 1;
        //} else if one_bigger_idx.is_some() && one_bigger_idx.unwrap() > &a && one_bigger_idx.unwrap() <= &b {
        //    println!("BBB {}", one_bigger_idx.unwrap());
        //    round += 1;
        //} else if one_less_idx.is_some() && one_less_idx.unwrap() < &b && one_less_idx.unwrap() >= &a {
        //    println!("CCC {}", one_less_idx.unwrap());
        //    round -= 1;
        //} else if one_less_idx.is_some() && one_less_idx.unwrap() > &a && one_less_idx.unwrap() <= &b {
        //    println!("DDD {}", one_less_idx.unwrap());
        //    round += 1;
        //};

        //if a > 0 && orig_b > 1 {
        //    if indexes[&(orig_b - 1)] > a && indexes[&(orig_b - 1)] <= b {
        //        println!("A:Minus 1 {} > {} && {} <= {}", indexes[&(orig_b - 1)], a, indexes[&(orig_b - 1)], b);
        //        round -= 1;
        //} else if b < (n as i32 - 1) as usize && orig_a < n as i32 {
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

//fn collect_numbers(arr: &[i32]) -> usize {
//    let mut set: BTreeSet<i32> = BTreeSet::new();
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
