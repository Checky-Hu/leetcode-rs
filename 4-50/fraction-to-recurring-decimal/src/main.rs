use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut result: String = String::new();
        let mut a: i64 = numerator as i64;
        let mut b: i64 = denominator as i64;
	if numerator == 0 {
	    result.push('0');
	    return result
	} else if numerator < 0 {
	    a = 0 - a;
	    if denominator > 0 {
	        result.push('-');
	    } else {
	        b = 0 - b;
	    }
	} else {
	    if denominator < 0 {
	        b = 0 - b;
	        result.push('-');
	    }
	}

	let mut tmp: i64 = a / b;
	result.push_str(&tmp.to_string());
	let mut pre_val: i64 = a - tmp * b;
	if pre_val == 0 {
	    result
	} else {
	    result.push('.');
	    let mut pre_vals: Vec<i64> = Vec::new();
	    loop {
	        pre_val *= 10;
		tmp = pre_val / b;
		if pre_val == tmp * b {
		    result.push_str(&tmp.to_string());
		    break;
		} else {
		    let mut i: usize = 0;
		    while i < pre_vals.len() {
			if pre_vals[i] == pre_val {
			    break;
			}
			i += 1;
		    }
		    if i < pre_vals.len() {
			result.insert(result.len() - pre_vals.len() + i, '(');
			result.push(')');
			break;
		    } else {
		        result.push_str(&tmp.to_string());
		        pre_vals.push(pre_val);
	                pre_val -= tmp * b;
		    }
		}
	    }
	    result
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut numerator: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => numerator = i32::from_str(&arg).expect("Error parse."),
	    2 => {
                ret = index;
                let denominator: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("String: {}", Solution::fraction_to_decimal(numerator, denominator));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
