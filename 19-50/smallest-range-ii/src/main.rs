extern crate quicksort;

use quicksort::qsi32;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn smallest_range_ii(a: Vec<i32>, k: i32) -> i32 {
        let len: usize = a.len();
        let mut t: Vec<i32> = a;
        qsi32::quick_sort(&mut t, 0, len - 1);
        let left: i32 = t[0] + k;
        let right: i32 = t[len - 1] - k;
        let mut result: i32 = t[len - 1] - t[0];
        for i in 0..(len - 1) {
            let mut max: i32 = t[i] + k;
            if max < right {
                max = right;
            }
            let mut min: i32 = t[i + 1] - k;
            if min > left {
                min = left;
            }
            let cur: i32 = max - min;
            if cur < result {
                result = cur;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = -1;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if -1 == k || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Range: {}", Solution::smallest_range_ii(a, k));
}
