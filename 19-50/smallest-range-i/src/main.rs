use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
	let mut min: i32 = a[0];
	let mut max: i32 = min;
	let mut i: usize = 1;
	while i < a.len() {
	    if a[i] < min {
	        min = a[i];
	    }
	    if a[i] > max {
	        max = a[i];
	    }
	    i += 1;
	}
	if max - min <= 2 * k {
	    0
	} else {
	    max - min - 2 * k
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = -1;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    1 => k = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
	        a.push(n);
	    },
	}
    }

    if -1 == k || 0 == ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Range: {}", Solution::smallest_range_i(a, k));
}

