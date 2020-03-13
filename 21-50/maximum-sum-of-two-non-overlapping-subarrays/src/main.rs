use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        let len: usize = a.len();
        let mut sum_l: Vec<i32> = vec![0; len + 1];
        let mut sum_m: Vec<i32> = vec![0; len + 1];
        for i in 0..len {
            if i < l as usize {
                sum_l[i + 1] = sum_l[i] + a[i];
            } else {
                sum_l[i + 1] = sum_l[i] + a[i] - a[i - l as usize];
            }
            if i < m as usize {
                sum_m[i + 1] = sum_m[i] + a[i];
            } else {
                sum_m[i + 1] = sum_m[i] + a[i] - a[i - m as usize];
            }
        }
        let mut result: i32 = 0;
        for i in 0..=(len - l as usize) {
            let left_1: usize = i;
            let right_1: usize = i + l as usize - 1;
            for j in 0..=(len - m as usize) {
                if j > right_1 || j + m as usize - 1 < left_1 {
                    let t: i32 = sum_l[i + l as usize] + sum_m[j + m as usize];
                    if t > result {
                        result = t;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut l: i32 = 0;
    let mut m: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => l = i32::from_str(&arg).expect("Error parse."),
            2 => m = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Maximum sum of two non-overlapping subarrays: {}",
        Solution::max_sum_two_no_overlap(a, l, m)
    );
}
