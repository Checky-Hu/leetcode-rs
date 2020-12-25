use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let target: usize = k as usize;
        let source: usize = nums.len();
        let mut result: Vec<i32> = Vec::with_capacity(target);
        for (i, v) in nums.iter().enumerate() {
            while let Some(x) = result.last() {
                if *x <= *v {
                    break;
                } else if result.len() + source - i > target {
                    result.pop();
                } else {
                    break;
                }
            }
            if result.len() < target {
                result.push(*v);
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
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

    let result: Vec<i32> = Solution::most_competitive(nums, k);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
