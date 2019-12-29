extern crate quicksort;

use quicksort::qsvec;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let len: usize = pairs.len();
        if len == 0 {
            return 0;
        }
        let mut tmp: Vec<Vec<i32>> = pairs;
        qsvec::quick_sort(&mut tmp, 0, len - 1);
        let mut pre_n: i32 = tmp[0][1];
        let mut result: i32 = 1;
        for p in tmp.iter().skip(1) {
            if p[0] > pre_n {
                result += 1;
                pre_n = p[1];
            } else if p[1] < pre_n {
                pre_n = p[1];
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut pairs: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    pairs.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Longest chain: {}", Solution::find_longest_chain(pairs));
}
