extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = nums.len() - 1;
        let mut tmp: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut tmp, 0, j);
        let mut result: i32 = 0;
        while i < j {
            result += tmp[j] - tmp[i];
            i += 1;
            j -= 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Min moves: {}", Solution::min_moves2(nums));
}
