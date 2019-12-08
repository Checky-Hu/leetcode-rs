use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        while left < right {
            let mid: usize = left + (right - left) / 2;
            if mid > 0 && nums[mid] == nums[mid - 1] {
                if (right - mid) & 1 == 0 {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid] == nums[mid + 1] {
                if (mid - left) & 1 == 0 {
                    left = mid + 2;
                } else {
                    right = mid - 1;
                }
            } else {
                return nums[mid]
            }
        }
        nums[right]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Single element: {}", Solution::single_non_duplicate(nums));
}
