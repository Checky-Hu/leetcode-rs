use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let len: usize = nums.len();
        if len == 0 {
            return true;
        }
        let mut status: bool = false;
        for i in 1..len {
            if nums[i] < nums[i - 1] {
                if status {
                    return false;
                } else {
                    status = true;
                }
            }
        }
        !(status && nums[0] < nums[len - 1])
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

    println!("Is sorted and rotated: {}", Solution::check(nums));
}
