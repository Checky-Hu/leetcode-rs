extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len: usize = citations.len();
	if len == 0 {
	    return 0
	}

        let mut tmp: Vec<i32> = citations.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);

	let mut i: usize = 1;
	while i <= len {
	    if tmp[len - i] >= i as i32 {
	        i += 1;
	    } else {
	        break;
	    }
	}
	i as i32 - 1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut citations: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    citations.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("h-index: {}", Solution::h_index(citations));
}
