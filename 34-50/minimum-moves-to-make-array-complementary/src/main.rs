use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let max: usize = limit as usize;
        let len: usize = nums.len();
        let mut prefix: Vec<i32> = vec![0; (max << 1) + 2];
        for i in 0..(len >> 1) {
            let (less, greater) = if nums[i] < nums[len - 1 - i] {
                (nums[i], nums[len - 1 - i])
            } else {
                (nums[len - 1 - i], nums[i])
            };
            prefix[1] += 2;
            prefix[less as usize + 1] -= 1;
            prefix[greater as usize + max + 1] += 1;
            prefix[(less + greater) as usize] -= 1;
            prefix[(less + greater) as usize + 1] += 1;
        }
        let mut result: i32 = i32::max_value();
        for i in 1..=(max << 1) {
            prefix[i] += prefix[i - 1];
            if prefix[i] < result {
                result = prefix[i];
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut limit: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => limit = i32::from_str(&arg).expect("Error parse."),
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

    println!("Minimum moves: {}", Solution::min_moves(nums, limit));
}
