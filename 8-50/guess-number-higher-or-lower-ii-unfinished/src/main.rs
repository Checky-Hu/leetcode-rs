use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let mut s: i32 = 1;
	let mut result: i32 = 0;
	while s <= n {
	    let tmp: i32 = s + (n - s) / 2;
	    if tmp == n {
	        break;
	    } else {
	        result += tmp;
	        s = tmp + 1;
	    }
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
            println!("Amount: {}", Solution::get_money_amount(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
