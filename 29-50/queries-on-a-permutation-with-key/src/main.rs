use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut array: Vec<i32> = Vec::with_capacity(m as usize);
        for i in 1..=m {
            array.push(i);
        }
        let mut result: Vec<i32> = Vec::new();
        for query in queries.iter() {
            let mut index: usize = 0;
            for (i, v) in array.iter().enumerate() {
                if *v == *query {
                    index = i;
                    break;
                }
            }
            array.remove(index);
            array.insert(0, *query);
            result.push(index as i32);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: i32 = 0;
    let mut queries: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                queries.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<i32> = Solution::process_queries(queries, m);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
