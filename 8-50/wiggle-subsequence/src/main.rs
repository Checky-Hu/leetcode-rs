use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        if len == 0 {
            return 0
        }
        let mut positive: i32 = 1;
        let mut negative: i32 = 1;
        let mut i: usize = 1;
        while i < len {
            if nums[i] > nums[i - 1] {
                positive = negative + 1;
            } else if nums[i] < nums[i - 1] {
                negative = positive + 1;
            }
            i += 1;
        }
        if positive >= negative {
            positive
        } else {
            negative
        }
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
        println!("Require at least one parameter.");
        return
    }

    println!("Wiggle max length: {}", Solution::wiggle_max_length(nums));
}
