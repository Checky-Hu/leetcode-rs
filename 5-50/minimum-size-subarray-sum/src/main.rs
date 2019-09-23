use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut result: i32 = 0;
        let mut flags: Vec<i32> = vec![0; len + 1];
        for i in 1..=len {
            flags[i] = flags[i - 1] + nums[i - 1];
        }
        for i in 0..len {
            let mut left: usize = i + 1;
            let mut right: usize = len;
            let target: i32 = flags[i] + s;
            while left <= right {
                let mid: usize = left + (right - left) / 2;
                if flags[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            if left == len + 1 {
                break;
            } else {
                if result == 0 || result as usize > left - i {
                    result = (left - i) as i32;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Min size: {}", Solution::min_sub_array_len(s, nums));
}
