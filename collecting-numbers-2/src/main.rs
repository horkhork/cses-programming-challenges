use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

// Tests
// cat <<EOF | cargo run
// 5 3
// 4 5 1 2 3
// 2 3
// 1 5
// 2 4
// EOF
//
// cat <<EOF | cargo run
// 5 3
// 4 2 1 5 3
// 2 3
// 1 5
// 2 4
// EOF
//
// cat <<EOF | cargo run
// 5 3
// 4 2 1 5 3
// 2 3
// 1 5
// 2 3
// EOF
//
// cat <<EOF | cargo run
// 20 200000
// 14 2 3 10 8 9 1 19 4 15 5 17 13 16 7 11 6 12 18 20
// 13 13
// 10 1
// 12 20
// 4 15
// 8 6
// EOF

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

    let mut arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    //println!("Arr:{:?}", arr);

    let mut indexes: BTreeMap<i32, usize> = BTreeMap::new();
    let mut round: i32 = arr
        .iter()
        .enumerate()
        .map(|(i, x)| {
            indexes.insert(*x, i);
            match indexes.contains_key(&(x - 1)) {
                true => 0,
                false => 1,
            }
        })
        .sum();
    println!("N:{}", n);
    println!("Arr:{:?}", arr);
    println!("Indexes:{:?}", indexes);
    println!("Initial Count:{:?}", round);

    for line in lines {
        // Get the next values for a and b, rebased to zero and ordered
        let (a, b) = match line
            .unwrap()
            .split_whitespace()
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<Vec<usize>>()[..]
        {
            // Zero-base indexes and ensure that a < b (ordering doesn't matter for the swaps)
            [a, b] => match a < b {
                true => (a - 1, b - 1),
                false => (b - 1, a - 1),
            },
            _ => panic!("line not valid"),
        };

        // Keep track of the original values for calculations
        let a0 = arr[a];
        let b0 = arr[b];

        println!( "\nProcessing {:?}::: IdxA:{} IdxB:{} ValA:{} ValB:{}", arr, a, b, a0, b0);

        // Do the actual Swap
        arr.swap(a, b);

        if b - a == 1 {
            // Easy case, handle when indexes a and b are adjecent
            println!("adjacent swap");
            //match a0 < b0 {
            //    true => round += 1,
            //    false => round -= 1,
            //}
            indexes = BTreeMap::new();
            round = arr
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    indexes.insert(*x, i);
                    match indexes.contains_key(&(x - 1)) {
                        true => 0,
                        false => 1,
                    }
                })
                .sum();
        } else {
            // Handle the case when the values for a and b are monotonic neighbors
            round += if (a0 - b0).abs() == 1 {
                println!("monotonic neighbor swap");
                match a0 < b0 {
                    true => 1,
                    false => -1,
                }
            } else {
                0
            };
            round += match a0 {
                // Original A value == 1
                x if x == 1 => {
                    let one_more = x + 1;
                    let one_more_idx = indexes[&one_more];
                    println!( "---Orig A == 1 val {} (,{}); Idx:{} < IdxB:{}", x, one_more, one_more_idx, b);
                    if one_more_idx > a && one_more_idx < b {
                        println!("AAA plus 1");
                        1
                    } else {
                        println!("AAA zero");
                        0
                    }
                }
                // Original A value == N
                x if x == n => {
                    let one_less = x - 1;
                    let one_less_idx = indexes[&one_less];
                    println!( "---Orig A END val {} (,{}); Idx:{} < IdxB:{}", x, one_less, one_less_idx, b);
                    if one_less_idx > a && one_less_idx < b {
                        println!("EEE plus 1");
                        -1
                    } else {
                        0
                    }
                }
                // Original A value in the middle 1 < x < N
                x => {
                    let one_less = x - 1;
                    let one_less_idx = indexes[&one_less];
                    let one_more = x + 1;
                    let one_more_idx = indexes[&one_more];
                    println!( "Orig A middle val {} ({},{}); -idx:{} > a:{} && +idx:{} < b:{}", x, one_less, one_more, one_less_idx, a, one_less_idx, b);
                    if one_less_idx > a && one_less_idx < b {
                        println!("BBB minus 1");
                        -1
                    } else if one_more_idx > a && one_more_idx < b {
                        println!("BBB minus 1");
                        1
                    } else {
                        println!("BBB zero");
                        0
                    }
                }
            };
            round += match b0 {
                // Original B value == 1
                x if x == 1 => {
                    println!("Orig B == 1");
                    let one_more = x + 1;
                    let one_more_idx = indexes[&one_more];
                    println!( "---Orig B middle val {} (,{}); {} > {}", x, one_more, one_more_idx, a);
                    if one_more_idx > a && one_more_idx < b{
                        println!("CCC plus 1");
                        -1
                    } else {
                        println!("CCC zero");
                        0
                    }
                }
                // Original B value == N
                x if x == n => {
                    let one_less = x - 1;
                    let one_less_idx = indexes[&one_less];
                    println!( "---Orig B END val {} (,{}); Idx:{} < IdxB:{}", x, one_less, one_less_idx, b);
                    if one_less_idx > a && one_less_idx < b {
                        println!("FFF plus 1");
                        1
                    } else {
                        0
                    }
                }
                // Original A value in the middle 1 < x < N
                x => {
                    let one_less = x - 1;
                    let one_less_idx = indexes[&one_less];
                    let one_more = x + 1;
                    let one_more_idx = indexes[&one_more];
                    println!( "Orig B middle val {} ({},{}); -idx:{} <= a:{} && +idx:{} < b:{}", x, one_less, one_more, one_less_idx, a, one_more_idx, b);
                    if one_less_idx > a && one_less_idx < b {
                        1
                    } else if one_more_idx > a && one_more_idx < b {
                        -1
                    } else {
                        0
                    }
                }
            };
        }
        println!("Arr:{:?} === {}", arr, round);
        println!("{}", round);

        // Update where our indexes are for each value
        indexes.insert(arr[a], a);
        indexes.insert(arr[b], b);
        //println!("Indexes {:?}", indexes);
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
