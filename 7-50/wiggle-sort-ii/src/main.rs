extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let len: usize = nums.len();
        let mut tmp: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        let mut front_i: usize = (len + 1) / 2;
        let mut back_i: usize = len;
        for i in 0..len {
            if i & 1 == 0 {
                front_i -= 1;
                nums[i] = tmp[front_i];
            } else {
                back_i -= 1;
                nums[i] = tmp[back_i];
            }
        }
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
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    Solution::wiggle_sort(&mut nums);
    for n in nums {
        print!("{} ", n);
    }
    print!("\n");
}
