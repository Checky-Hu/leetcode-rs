use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        // (min value end with index, min value)
        let mut min: (i32, i32) = (nums[0], nums[0]);
        // (max value end with index, max value)
        let mut max: (i32, i32) = (nums[0], nums[0]);
        for num in nums.iter().skip(1) {
            max.0 = if max.0 >= 0 { *num + max.0 } else { *num };
            if max.0 > max.1 {
                max.1 = max.0
            }
            min.0 = if min.0 >= 0 { *num } else { *num + min.0 };
            if min.0 < min.1 {
                min.1 = min.0
            }
        }
        let result: (i32, i32) = (min.1.abs(), max.1.abs());
        if result.0 > result.1 {
            result.0
        } else {
            result.1
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
        println!("Require at least 1 parameter.");
        return;
    }

    println!("maximum absolute sum: {}", Solution::max_absolute_sum(nums));
}
