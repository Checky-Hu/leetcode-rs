use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut current: i32 = 0;
        let mut prefix: i32 = 0;
        for v in nums.iter() {
            if *v <= prefix {
                if current > result {
                    result = current;
                }
                current = *v;
            } else {
                current += *v;
            }
            prefix = *v;
        }
        if current > result {
            result = current;
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

    println!(
        "Maximum ascending sum: {}",
        Solution::max_ascending_sum(nums)
    );
}
