extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let len: usize = nums.len();
        if len == 0 {
            return 0
        }
        let mut tmp_data: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut tmp_data, 0, len - 1);
        let mut dp: Vec<i32> = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target {
            for n in &tmp_data {
                if *n > i {
                    break;
                } else {
                    dp[i as usize] += dp[(i - *n) as usize];
                }
            }
        }
        dp[target as usize]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut target: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least two parameter.");
        return;
    }

    println!("Sum count: {}", Solution::combination_sum4(nums, target));
}
