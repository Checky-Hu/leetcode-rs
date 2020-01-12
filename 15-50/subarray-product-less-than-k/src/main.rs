use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut result: i32 = 0;
        let mut p: i32 = 1;
        let mut left: usize = 0;
        for (i, v) in nums.iter().enumerate() {
            p *= *v;
            while p >= k {
                p /= nums[left];
                left += 1;
            }
            result += (i - left) as i32 + 1;
        }
        result
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Num of subarray product less than k: {}",
        Solution::num_subarray_product_less_than_k(nums, k)
    );
}
