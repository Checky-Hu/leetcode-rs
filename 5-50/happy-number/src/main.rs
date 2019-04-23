use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut tmp: i32 = n;
	while tmp != 1 && tmp != 4 {
	    let mut sum: i32 = 0;
	    while tmp > 0 {
	        let v: i32 = tmp % 10;
		sum += v * v;
		tmp /= 10;
	    }
	    tmp = sum;
	}
	if tmp == 1 {
	    true
	} else {
	    false
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Happy: {}", Solution::is_happy(n));
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
