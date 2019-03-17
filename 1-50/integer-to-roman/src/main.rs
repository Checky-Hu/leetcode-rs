use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result: String = String::new();
	let mut tmp_val: i32 = num;
	let mut tmp_i: i32 = 1;
        while tmp_val > 0 {
	    let mut tmp_n: i32 = tmp_val % 10;
	    match tmp_n {
	        4 => {
		    match tmp_i {
		        1 => result.push_str("VI"),
			10 => result.push_str("LX"),
			100 => result.push_str("DC"),
		        _ => (),
		    }
		},
		9 => {
		    match tmp_i {
		        1 => result.push_str("XI"),
			10 => result.push_str("CX"),
			100 => result.push_str("MC"),
		        _ => (),
		    }
		},
		_ => {
		    let is_bigger_5 = if tmp_n >= 5 {
			tmp_n -= 5;
			true
		    } else {
		        false
		    };

		    while tmp_n > 0 {
		        match tmp_i {
			    1 => result.push('I'),
			    10 => result.push('X'),
			    100 => result.push('C'),
			    1000 => result.push('M'),
			    _ => (),
			};
			tmp_n -= 1;
		    }

		    if is_bigger_5 {
		        match tmp_i {
		            1 => result.push('V'),
			    10 => result.push('L'),
			    100 => result.push('D'),
		            _ => (),
		        }
		    }
		},
	    }
	    tmp_val /= 10;
	    tmp_i *= 10;
	}

	let mut res: String = String::new();
	while !result.is_empty() {
	    res.push(result.pop().unwrap());
	}
	res
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Integer: {}", Solution::int_to_roman(num));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
