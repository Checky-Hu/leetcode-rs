use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn subset_xor_sum_loop(nums: &[i32], index: usize, len: usize, current: i32) -> i32 {
        if index == len {
            current
        } else {
            Solution::subset_xor_sum_loop(nums, index + 1, len, current ^ nums[index])
                + Solution::subset_xor_sum_loop(nums, index + 1, len, current)
        }
    }

    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        Solution::subset_xor_sum_loop(&nums, 0, len, 0)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Sum of all subset xor totals: {}",
        Solution::subset_xor_sum(nums)
    );
}
