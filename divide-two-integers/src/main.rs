use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend != 0 {
	    let (is_same, tmp_dividend, tmp_divisor) = if dividend < 0 {
	        if divisor < 0 {
		    (true, dividend, divisor)
		} else {
		    (false, dividend, 0 - divisor)
		}
	    } else {
	        if divisor < 0 {
		    (false, 0 - dividend, divisor)
		} else {
		    (true, 0 - dividend, 0 - divisor)
		}
	    };

	    let mut sum: i64 = 0;
	    let mut result: i64 = 0;
	    while sum > tmp_dividend as i64 {
	        sum += tmp_divisor as i64;
		result += 1;
	    }
	    if sum != tmp_dividend as i64 {
	        result -= 1;
	    }
	    if  is_same {
	        if result > i32::max_value() as i64 {
		    i32::max_value()
		} else {
		    result as i32
		}
	    } else {
	        (0 - result) as i32
	    }
	} else {
	    0
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut dividend: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => dividend = i32::from_str(&arg).expect("Error parse."),
	    2 => {
                ret = index;
                let divisor: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Integer: {}", Solution::divide(dividend, divisor));
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
