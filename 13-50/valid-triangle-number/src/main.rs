extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        if len < 3 {
            return 0
        }
        let mut temp: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut temp, 0, len - 1);
        let mut result: i32 = 0;
        for i in 0..(len - 2) {
            for j in (i + 1)..(len - 1) {
                for k in (j + 1)..len {
                    if temp[i] + temp[j] > temp[k] {
                        result += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Triangle number: {}", Solution::triangle_number(nums));
}

