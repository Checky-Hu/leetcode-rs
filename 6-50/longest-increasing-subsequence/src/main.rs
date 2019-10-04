use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        if len == 0 {
            return 0
        }
        let mut result: Vec<i32> = vec![1; len];
        let mut max_len: i32 = 1;
        let mut i: usize = 1;
        while i < len {
            let mut j: usize = i - 1;
            loop {
                if nums[i] > nums[j] && result[i] < result[j] + 1 {
                    result[i] = result[j] + 1;
                } else if nums[i] == nums[j] && result[i] < result[j] {
                    result[i] = result[j];
                }
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            if result[i] > max_len {
                max_len = result[i];
            }
            i += 1;
        }
        max_len
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Length of lis: {}", Solution::length_of_lis(nums));
}
