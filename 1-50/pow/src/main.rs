use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: f64 = 0.0f64;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => x = f64::from_str(&arg).expect("Error parse."),
	    2 => {
	        let n: i32 = i32::from_str(&arg).expect("Error parse.");
                ret = index;
                println!("Value: {}", Solution::my_pow(x, n));
		break;
	    },
	    _ => (),

	}
    }

    match ret {
        0 => println!("Require at least two parameter."),
	_ => (),
    }
}
