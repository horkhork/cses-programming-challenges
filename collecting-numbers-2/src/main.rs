use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();

    // Parse the first line, unused
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

        if let Some(&i) = indexes.get(&(arr[a] - 1)) {
            if a < i && i <= b {
                rounds -= 1;
            }
        }
        if let Some(&i) = indexes.get(&(arr[a] + 1)) {
            if a < i && i <= b {
                rounds += 1;
            }
        }
        if let Some(&i) = indexes.get(&(arr[b] - 1)) {
            if a < i && i <= b {
                rounds += 1;
            }
        }
        if let Some(&i) = indexes.get(&(arr[b] + 1)) {
            if a < i && i <= b {
                rounds -= 1;
            }
        }

        // Do the actual Swap
        arr.swap(a, b);

        // Update where our indexes are for each value
        indexes.insert(arr[a], a);
        indexes.insert(arr[b], b);

        println!("{}", rounds);
    }
}
