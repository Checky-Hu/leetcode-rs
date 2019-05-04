use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        let tmp: i32 = n % 4;
	if tmp == 0 {
	    false
	} else {
	    true
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Win: {}", Solution::can_win_nim(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
