extern crate quicksort;

use quicksort::qsi32;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let len: usize = a.len();
        let mut tmp: Vec<i32> = a;
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        let mut result: Vec<i32> = Vec::with_capacity(len);
        let mut visits: Vec<bool> = vec![false; len];
        let mut index: usize = 0;
        for v in b {
            let mut new_i: usize = len;
            let mut found: bool = false;
            for i in index..len {
                if !visits[i] {
                    if new_i > i {
                        new_i = i;
                    }
                    if tmp[i] > v {
                        visits[i] = true;
                        result.push(tmp[i]);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                visits[new_i] = true;
                result.push(tmp[new_i]);
                index = new_i;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if a.len() == n as usize {
                    b.push(t);
                } else {
                    a.push(t);
                }
            }
        }
    }

    if 0 == n || 2 * n != ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::advantage_count(a, b);
    for r in &result {
        print!("{} ", *r);
    }
    println!();
}
