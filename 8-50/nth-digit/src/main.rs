use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut bits: i64 = 1;
	let mut count: i64 = 9;
	let mut sum: i64 = n.into();
	while sum > bits * count {
	    sum -= bits * count;
	    bits += 1;
	    count *= 10;
	}
	let mut num: i64 = (sum - 1) / bits + count / 9;
	let mut v: Vec<i64> = Vec::new();
	while num > 0 {
	    v.push(num % 10);
	    num /= 10;
	}
	let index: i64 = (sum - 1) % bits;
	v[v.len() - 1 - index as usize] as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Nth digit: {}", Solution::find_nth_digit(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
