extern crate quicksort;

use quicksort::qsi32;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        let mut result: Vec<i32> = nums;
        qsi32::quick_sort(&mut result, 0, len - 1);
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
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::sort_array(nums);
    for n in &result {
        print!("{} ", *n);
    }
    println!();
}
