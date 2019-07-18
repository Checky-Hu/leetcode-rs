use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn rotate_digits(n: i32) -> i32 {
        let mut result: i32 = 0;
	for num in 1..=n {
	    let mut is_valid: bool = false;
            let mut tmp_num: i32 = num;
	    while tmp_num > 0 {
	        let t: i32 = tmp_num % 10;
	        match t {
	            0 | 1 | 8 => (),
		    2 | 5 | 6 | 9 => is_valid = true,
		    _ => {
		        is_valid = false;
			break;
		    },
	        }
	        tmp_num /= 10;
	    }
	    if is_valid {
	        result += 1;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    println!("Count: {}", Solution::rotate_digits(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

