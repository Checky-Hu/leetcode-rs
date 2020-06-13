use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut result: usize = 0;
        let mut max: Vec<i32> = Vec::new();
        let mut min: Vec<i32> = Vec::new();
        let mut left: usize = 0;
        for (i, v) in nums.iter().enumerate() {
            while let Some(x) = max.last() {
                if *x < *v {
                    max.pop();
                } else {
                    break;
                }
            }
            max.push(*v);
            while let Some(x) = min.last() {
                if *x > *v {
                    min.pop();
                } else {
                    break;
                }
            }
            min.push(*v);
            while max[0] - min[0] > limit {
                if max[0] == nums[left] {
                    max.remove(0);
                }
                if min[0] == nums[left] {
                    min.remove(0);
                }
                left += 1;
            }
            let t: usize = i - left + 1;
            if t > result {
                result = t;
            }
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut limit: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => limit = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Longest subarray: {}",
        Solution::longest_subarray(nums, limit)
    );
}
