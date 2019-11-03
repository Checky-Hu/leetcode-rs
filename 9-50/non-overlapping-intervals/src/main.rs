extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsvec;

struct Solution {
}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let len: usize = intervals.len();
        if len == 0 {
            return 0
        }
        let mut result: i32 = 0;
        let mut tmp: Vec<Vec<i32>> = intervals.clone();
        qsvec::quick_sort(&mut tmp, 0, len - 1);
        let mut pre_i: usize = 0;
        for i in 1..len {
            if tmp[pre_i][1] > tmp[i][0] {
                result += 1;
                if tmp[pre_i][1] > tmp[i][1] {
                    pre_i = i;
                }
            } else {
                pre_i = i;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut intervals: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        tmp.push(number);
		if tmp.len() == 2 {
		    intervals.push(tmp);
		    tmp = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || ret & 1 == 1 {
        println!("Require at least (2 * n) parameters.");
        return;
    }

    println!("Erase overlap: {}", Solution::erase_overlap_intervals(intervals));
}
