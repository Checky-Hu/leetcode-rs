use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut tmp: i32 = n;
	let mut count: i32 = 0;
	while tmp > 0 {
	    count += tmp & 1;
	    tmp >>= 1;
	}
	count == 1
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Power of Two: {}", Solution::is_power_of_two(n));
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
