extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let len: usize = heights.len();
        let mut tmp: Vec<i32> = heights.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
	let mut result: i32 = 0;
	let mut i: usize = 0;
        while i < len {
	    if heights[i] != tmp[i] {
	        result += 1;
	    }
	    i += 1;
        }
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut heights: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                heights.push(n);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Wrong positions: {}", Solution::height_checker(heights));
}

