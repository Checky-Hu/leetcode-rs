use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut num: i32 = 0;
        for v in nums.iter() {
            if *v <= num {
                num += 1;
                result += num - *v;
            } else {
                num = *v;
            }
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

    println!("Minimum operations: {}", Solution::min_operations(nums));
}
