use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
        let len: usize = a.len();
        let mut sum: Vec<i32> = vec![0; len + 1];
        for (i, v) in a.iter().enumerate() {
            sum[i + 1] = sum[i] + *v;
        }
        let mut fun: Vec<i32> = vec![0; len + 1];
        fun[0] = 1;
        let mut result: i32 = 0;
        for i in 1..=len {
            if sum[i] >= s {
                result += fun[(sum[i] - s) as usize];
            }
            fun[sum[i] as usize] += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: i32 = -1;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if -1 == s || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Number of subarrays: {}",
        Solution::num_subarrays_with_sum(a, s)
    );
}
