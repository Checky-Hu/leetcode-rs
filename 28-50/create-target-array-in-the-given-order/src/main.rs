use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        let mut result: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            result.insert(index[i] as usize, nums[i]);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut len: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut index: Vec<i32> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        match i {
            0 => (),
            1 => len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret > len {
                    index.push(n);
                } else {
                    nums.push(n);
                }
            }
        }
    }

    if 0 == len || 2 * len != ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::create_target_array(nums, index);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
