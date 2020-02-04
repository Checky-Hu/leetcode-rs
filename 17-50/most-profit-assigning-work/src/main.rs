extern crate quicksort;

use quicksort::qstuple;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let len: usize = difficulty.len();
        let mut tuples: Vec<(i32, i32)> = Vec::with_capacity(len);
        for i in 0..len {
            tuples.push((difficulty[i], profit[i]));
        }
        qstuple::quick_sort(&mut tuples, 0, len - 1);
        let mut result: i32 = 0;
        for w in worker {
            let mut tmp: i32 = 0;
            for v in &tuples {
                if (*v).0 <= w {
                    if (*v).1 > tmp {
                        tmp = (*v).1;
                    }
                } else {
                    break;
                }
            }
            result += tmp;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut w: i32 = 0;
    let mut difficulty: Vec<i32> = Vec::new();
    let mut profit: Vec<i32> = Vec::new();
    let mut worker: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => w = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if difficulty.len() != n as usize {
                    difficulty.push(t);
                } else if profit.len() != n as usize {
                    profit.push(t);
                } else {
                    worker.push(t);
                }
            }
        }
    }

    if 0 == n || 0 == w || 2 * n + w != ret {
        println!("Require at least (2 + 2 * arg1 + arg2) parameters.");
        return;
    }

    println!(
        "Max profit: {}",
        Solution::max_profit_assignment(difficulty, profit, worker)
    );
}
