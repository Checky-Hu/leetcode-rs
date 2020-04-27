use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut odd_indices: Vec<usize> = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            if *n & 1 == 1 {
                odd_indices.push(i);
            }
        }
        let len: usize = odd_indices.len();
        if len < k as usize {
            return 0;
        }
        let mut result: usize = 0;
        for i in 0..len {
            if i + 1 >= k as usize {
                let s: usize = if i >= k as usize {
                    odd_indices[i - k as usize] + 1
                } else {
                    0
                };
                let e: usize = if i + 1 >= len {
                    nums.len() - 1
                } else {
                    odd_indices[i + 1] - 1
                };
                result += (odd_indices[i + 1 - k as usize] + 1 - s) * (e + 1 - odd_indices[i]);
            }
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Number of nice subarrays: {}",
        Solution::number_of_subarrays(nums, k)
    );
}
