extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let len: usize = nums.len();
        let mut tmp: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        tmp[len - k as usize]
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            },
        }
    }

    if 0 == ret || k <= 0 || k > ret {
        println!("Require at least (len + 1) parameter.");
        return;
    }

    println!("Kth largest: {}", Solution::find_kth_largest(nums, k));
}
