use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let len: usize = nums.len();
        let mut i: usize = 0;
        while i < len {
            for _j in 0..nums[i] {
                result.push(nums[i + 1]);
            }
            i += 2;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least odd parameters.");
        return;
    }

    let result: Vec<i32> = Solution::decompress_rl_elist(nums);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
