use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let max: i32 = 2_i32.pow(maximum_bit as u32) - 1;
        let len: usize = nums.len();
        let mut result: Vec<i32> = vec![0; len];
        let mut current: i32 = 0;
        for (i, num) in nums.iter().enumerate() {
            current ^= *num;
            result[len - 1 - i] = current ^ max;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut maximum_bit: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => maximum_bit = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result = Solution::get_maximum_xor(nums, maximum_bit);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
