use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let mut result: i32 = 0;
        let mut sum: i32 = 0;
        let mut cur_max: i32 = 0;
        for v in arr.iter() {
            sum += *v;
            cur_max = if cur_max > 0 { cur_max + *v } else { *v };
            if cur_max > result {
                result = cur_max;
            }
        }
        if k == 1 {
            return result;
        }
        let mut left: i32 = 0;
        let mut cur_sum: i32 = 0;
        for v in arr.iter() {
            cur_sum += *v;
            if cur_sum > left {
                left = cur_sum;
            }
        }
        let mut right: i32 = 0;
        cur_sum = 0;
        let mut i: usize = arr.len() - 1;
        loop {
            cur_sum += arr[i];
            if cur_sum > right {
                right = cur_sum;
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        let t: i64 = ((left + right) as i64 % modulo
            + if sum > 0 {
                (sum as i64 * (k - 2) as i64) % modulo
            } else {
                0
            })
            % modulo;
        if t as i32 > result {
            result = t as i32;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "K concatenation Maximum sum: {}",
        Solution::k_concatenation_max_sum(arr, k)
    );
}
