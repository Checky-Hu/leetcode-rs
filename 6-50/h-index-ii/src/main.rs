use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len: usize = citations.len();
	if len == 0 {
	    return 0
	}

	let mut s: usize = 0;
	let mut e: usize = len;
	let mut index: usize = s;
	while s <= e {
	    let mid: usize = s + (e - s) / 2;
	    if mid == 0 {
	        if citations[0] >= 0 {
		    index = mid;
		    s = mid + 1;
		} else {
		    break;
		}
	    } else if citations[len - mid] >= mid as i32 {
	        index = mid;
	        s = mid + 1;
	    } else {
	        if mid > 0 {
		    e = mid - 1;
		} else {
	            break;
		}
	    }
	}
	index as i32
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
