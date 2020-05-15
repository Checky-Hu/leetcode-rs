use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min: i32 = i32::max_value();
        let mut sum: i32 = 0;
        for n in nums.iter() {
            sum += *n;
            if sum < min {
                min = sum;
            }
        }
        let mut result: i32 = 1 - min;
        if result <= 0 {
            result = 1;
        }
        result
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Minimum start value: {}", Solution::min_start_value(nums));
}
