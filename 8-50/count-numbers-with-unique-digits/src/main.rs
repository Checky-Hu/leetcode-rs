use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut result: i32 = 1;
        let mut bits: i32 = 1;
	while bits <= n && bits <= 10 {
	    let mut next: i32 = 9;
	    let mut tmp: i32 = 9;
	    while next + bits > 10 {
	        tmp *= next;
	        next -= 1;
	    }
	    result += tmp;
	    bits += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Count: {}", Solution::count_numbers_with_unique_digits(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
