extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsvec;

struct Solution {
}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let len: usize = intervals.len();
        if len == 0 {
            return result
        }
        let mut tmp: Vec<Vec<i32>> = Vec::with_capacity(len);
        let mut i: i32 = 0;
        for v in &intervals {
            tmp.push(vec![v[0], i]);
            i += 1;
        }
        qsvec::quick_sort(&mut tmp, 0, len - 1);
        for v in &intervals {
            let mut t: i32 = -1;
            for cup in &tmp {
                if cup[0] >= v[1] {
                    t = cup[1];
                    break;
                }
            }
            result.push(t);
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

    let result: Vec<i32> = Solution::find_right_interval(intervals);
    for i in result {
        print!("{} ", i);
    }
    print!("\n");
}
