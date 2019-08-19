extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let len: usize = a.len();
        let mut tmp: Vec<i32> = a.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        let mut i: usize = 0;
        while i < len && tmp[i] < 0 {
            i += 1;
        }
        let mut result: i32 = 0;
        if i >= k as usize {
            let mut j: i32 = 1;
            for n in tmp {
                if j <= k {
                    result -= n;
                } else {
                    result += n;
                }
                j += 1;
            }
        } else {
            let find_min: bool = if 1 & (k as usize - i) == 0 {
                false
            } else {
                true
            };
            let mut j: usize = 0;
            let mut min: i32 = i32::max_value();
            for n in tmp {
                if j < i {
                    result -= n;
                    if find_min && 0 - n < min {
                        min = 0 - n;
                    }
                } else {
                    result += n;
                    if find_min && n < min {
                        min = n;
                    }
                }
                j += 1;
            }
            if find_min {
                result -= 2 * min;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            },
        }
    }

    if 0 == ret || 0 == k {
        println!("Require at least two parameters.");
        return;
    }

    println!("Largest sum: {}", Solution::largest_sum_after_k_negations(a, k));
}

