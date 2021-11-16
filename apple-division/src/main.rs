use std::collections::{BTreeSet, BTreeMap};
use std::io::{BufRead, BufReader};
use std::fmt;

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    //let mut split = line.split_whitespace();
    //let _n: i32 = split.next().unwrap().parse().unwrap();

    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut values = Vec::new();
    for val in line.split_whitespace() {
        let val = val.parse::<i32>().unwrap();
        values.push(val);
    }
    println!("values {:?}", values);
    let test = BTreeSet::from([1,1,2,2,3,3]);
    println!("test {:?}", test);
    //let sum = values.iter().sum();
    //let apples = Apples{
    //    group1: BTreeMap::from([(sum, values)]),
    //    group2: BTreeMap::new(),
    //};
    //println!("apples {}", apples);
    //subset_sums(values.into_iter().collect(), sum / 2, vec![]);

    //combinate_vec(apples.group1[&sum].clone(), vec![]);
    let len = if values.len() % 2 == 0 {
        values.len() / 2
    } else {
        (values.len() / 2) + 1
    };
    combinate_vec(vec![], values.clone(), len, values);
}

struct Apples {
    // Maps the sum to the component weights
    group1: BTreeMap<i32, Vec<i32>>,
    group2: BTreeMap<i32, Vec<i32>>,
}

impl fmt::Display for Apples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.group1, self.group2)
    }
}

//fn combinate_vec(mut accum: Vec<i32>, rest: Vec<i32>, n: usize) -> i32 {
//    println!("Accum:{:?} Rest:{:?} N:{:?}", accum, rest, n);
//    if n == 0 {
//        println!("Done Accum:{:?} Rest:{:?}", accum, rest);
//        let pop = accum.pop().unwrap();
//        println!("Pop:{}", pop);
//        return pop;
//    } else {
//        //for i in 0..rest.len() {
//        let mut r = rest.clone();
//        for (i, _) in rest.iter().enumerate() {
//            //a.push(rest[i]);
//            let mut a = accum.clone();
//            a.push(r.pop().unwrap());
//            a.push(combinate_vec(a.clone(), r.to_vec(), (n-1) as usize));
//        }
//        return -1;
//    }
//}
fn combinate_vec(accum: Vec<i32>, rest: Vec<i32>, n: usize, all: Vec<i32>) {
    //println!("Accum:{:?} Rest:{:?} N:{:?}", accum, rest, n);
    if n == 0 {
        //println!("Done Accum:{:?} Rest:{:?}", accum, rest);
        let rest: Vec<_> = all.into_iter().filter(|x| !accum.contains(x)).collect();
        println!("Done Accum:{:?} Rest:{:?}", accum, rest);
    } else {
        for i in 0..rest.len() {
        //let mut r = rest.clone();
        //for (i, _) in rest.iter().enumerate() {
            let mut a = accum.clone();
            a.push(rest[i]);
            //let mut a = accum.clone();
            //a.push(r.pop().unwrap());
            combinate_vec(a, rest[i+1..].to_vec(), (n-1) as usize, all.clone());
        }
    }
}
//    static void combinations2(String[] arr, int len, int startPosition, String[] result){
//        if (len == 0){
//            System.out.println(Arrays.toString(result));
//            return;
//        }       
//        for (int i = startPosition; i <= arr.length-len; i++){
//            result[result.length - len] = arr[i];
//            combinations2(arr, len-1, i+1, result);
//        }
//    }      

//fn subset_sums(numbers: Vec<i32>, threshold: i32, part: Vec<i32>) {
//    let s: i32 = part.iter().sum();
//    let mut counts: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
//    if numbers.len() == 0 {
//        counts.insert(part.iter().sum(), part);
//        return ;
//    //if s == threshold {
//    //    println!("EQ threshold {} with {}", threshold, s);
//    //    counts.insert(0, part);
//    //    return counts;
//    //} else if s > threshold {
//    //    println!("Exceeded threshold {} with {}", threshold, s);
//    //    let over = s - threshold;
//    //    counts.insert(over, part);
//    //    return counts;
//    //}
//    }
//
//    for (i, n) in numbers.iter().enumerate() {
//        let mut rem = Vec::new();
//        rem.extend_from_slice(&numbers[i+1..]);
//        let mut p = Vec::new();
//        p.extend_from_slice(&part);
//        p.push(*n);
//        let mut ret = subset_sums(rem, threshold, p);
//
//        //println!("list {} {} {:?}+{}", (part.iter().sum::<i32>() + n)-threshold, threshold, part, n);
//    }
//
//}
