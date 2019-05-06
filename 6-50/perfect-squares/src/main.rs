use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut tmp: i32 = n;
	while tmp % 4 == 0 {
	    tmp /= 4;
	}
	if tmp % 8 == 7 {
	    return 4
	}
	let mut i: i32 = 0;
	loop {
	    let product: i32 = i * i;
	    if product > tmp {
	        break;
	    }

	    let j: i32 = ((tmp - product) as f64).sqrt() as i32;
	    if product + j * j == tmp {
	        return if i == 0 {
		    1
		} else {
		    2
		}
	    } else {
	        i += 1;
	    }
	}
	3
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Num: {}", Solution::num_squares(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
