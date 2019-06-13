use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
	    n
	} else {
	    let mut fn_2: i32 = 0;
	    let mut fn_1: i32 = 1;
	    for _i in 2..=n {
	        let tmp: i32 = fn_1 + fn_2;
		fn_2 = fn_1;
		fn_1 = tmp;
	    }
	    fn_1
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("F({}): {}", n, Solution::fib(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
