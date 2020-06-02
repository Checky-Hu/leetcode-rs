use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let len: usize = nums.len();
        let mut pre: usize = len;
        for (i, v) in nums.iter().enumerate() {
            if *v == 1 {
                if pre != len && i - pre - 1 < k as usize {
                    return false;
                }
                pre = i;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("K length apart: {}", Solution::k_length_apart(nums, k));
}
