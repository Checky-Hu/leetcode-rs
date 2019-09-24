use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn try_rob(nums: &Vec<i32>, l: usize, r: usize) -> i32 {
        let mut rob: i32 = 0;
        let mut no_rob: i32 = 0;
        let mut i: usize = l;
        while i < r {
            let pre_rob: i32 = rob;
            let pre_no_rob: i32 = no_rob;
            rob = pre_no_rob + nums[i];
            no_rob = if pre_rob > pre_no_rob {
                pre_rob
            } else {
                pre_no_rob
            };
            i += 1;
        }
        if rob > no_rob {
            rob
        } else {
            no_rob
        }
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0
        }

        let len: usize = nums.len();
        if len == 1 {
            return nums[0]
        }
        let r1: i32 = Solution::try_rob(&nums, 0, len - 1);
        let r2: i32 = Solution::try_rob(&nums, 1, len);
        if r1 > r2 {
            r1
        } else {
            r2
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Money: {}", Solution::rob(nums));
}
