use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let len: usize = n as usize;
        let mut result: Vec<i32> = Vec::with_capacity(2 * len);
        for i in 0..len {
            result.push(nums[i]);
            result.push(nums[i + len]);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::shuffle(nums, n);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
