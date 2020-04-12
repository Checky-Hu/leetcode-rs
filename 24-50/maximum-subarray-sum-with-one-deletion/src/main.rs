use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let len: usize = arr.len();
        let mut prefix: i32 = 0;
        let mut left: Vec<i32> = Vec::with_capacity(len);
        let mut max: i32 = i32::min_value();
        for v in arr.iter() {
            prefix = if prefix > 0 { prefix + *v } else { *v };
            left.push(prefix);
            if *v > max {
                max = *v;
            }
        }
        prefix = 0;
        let mut right: Vec<i32> = vec![0; len];
        let mut i: usize = len - 1;
        loop {
            prefix = if prefix > 0 { prefix + arr[i] } else { arr[i] };
            right[i] = prefix;
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        let mut result: i32 = max;
        for i in 0..(len - 1) {
            let t: i32 = if i == 0 {
                right[i + 1]
            } else if i == len - 1 {
                left[i - 1]
            } else if left[i - 1] > 0 {
                if right[i + 1] > 0 {
                    left[i - 1] + right[i + 1]
                } else {
                    left[i - 1]
                }
            } else if right[i + 1] > 0 {
                right[i + 1]
            } else if left[i - 1] > right[i + 1] {
                left[i - 1]
            } else {
                right[i + 1]
            };
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Maximum subarray sum with one deletion: {}",
        Solution::maximum_sum(arr)
    );
}
