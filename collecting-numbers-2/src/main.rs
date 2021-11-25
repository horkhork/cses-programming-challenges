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

    let (_n, _m) = match lines
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

    // Parse the array of numbers
    let mut arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();

    // Keep track of each number in the array and it's initial index in the list
    let mut indexes: BTreeMap<i32, usize> = BTreeMap::new();
    // Get the number of rounds for the initial array
    let mut rounds: i32 = arr
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

    //println!("N:{}", n);
    //println!("Arr:{:?}", arr);
    //println!("Indexes:{:?}", indexes);
    //println!("Initial Count:{:?}", rounds);

    // For each remaining line process the swap and update the rounds
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

        let a_minus_1 = arr[a] - 1;
        let a_minus_1_idx = indexes.get(&a_minus_1);
        let a_plus_1 = arr[a] + 1;
        let a_plus_1_idx = indexes.get(&a_plus_1);
        let b_minus_1 = arr[b] - 1;
        let b_minus_1_idx = indexes.get(&b_minus_1);
        let b_plus_1 = arr[b] + 1;
        let b_plus_1_idx = indexes.get(&b_plus_1);

        if let Some(&i) = a_minus_1_idx {
            if a < i && i <= b {
                rounds -= 1;
            }
        }
        if let Some(&i) = a_plus_1_idx {
            if a < i && i <= b {
                rounds += 1;
            }
        }
        if let Some(&i) = b_minus_1_idx {
            if a < i && i <= b {
                rounds += 1;
            }
        }
        if let Some(&i) = b_plus_1_idx {
            if a < i && i <= b {
                rounds -= 1;
            }
        }

        // Do the actual Swap
        arr.swap(a, b);

        println!("{}", rounds);

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
