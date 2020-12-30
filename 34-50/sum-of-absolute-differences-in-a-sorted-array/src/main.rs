use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut diff: i32 = 0;
        let len: usize = nums.len();
        for i in 1..len {
            diff += nums[i] - nums[0];
        }
        let mut result: Vec<i32> = Vec::with_capacity(len);
        result.push(diff);
        for i in 1..len {
            if 2 * i >= len {
                diff += (2 * i - len) as i32 * (nums[i] - nums[i - 1]);
            } else {
                diff -= (len - 2 * i) as i32 * (nums[i] - nums[i - 1]);
            }
            result.push(diff);
        }
        result
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

    let result: Vec<i32> = Solution::get_sum_absolute_differences(nums);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
