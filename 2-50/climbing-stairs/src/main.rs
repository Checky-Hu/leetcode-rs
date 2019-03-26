use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
	    1
	} else if n == 2 {
	    2
	} else {
	    let mut f_n_1: i32 = 2;
	    let mut f_n_2: i32 = 1;
	    for _i in 3..=n {
	        let tmp: i32 = f_n_1;
		f_n_1 += f_n_2;
		f_n_2 = tmp;
	    }
	    f_n_1
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Integer: {}", Solution::climb_stairs(n));
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
