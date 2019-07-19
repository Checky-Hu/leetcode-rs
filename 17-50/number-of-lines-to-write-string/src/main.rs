use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; 2];
	for c in s.chars() {
	    let tmp: i32 = result[1] + widths[(c as u8 - 97) as usize];
	    if tmp < 100 {
	        result[1] += widths[(c as u8 - 97) as usize];
	    } else {
	        result[0] += 1;
	        if tmp == 100 {
	            result[1] = 0;
	        } else {
	            result[1] = widths[(c as u8 - 97) as usize];
		}
	    }
	}
	if result[1] > 0 {
	    result[0] += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    let mut widths: Vec<i32> = vec![0; 26];
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => s = arg,
	    _ => {
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        widths[ret] = number;
                ret += 1;
		if ret == 26 {
		    break;
		}
	    },
	}
    }

    if 26 != ret {
        println!("Require at least 27 parameters.");
	return;
    }

    let result: Vec<i32> = Solution::number_of_lines(widths, s);
    println!("Lines: {}, Last count: {}", result[0], result[1]);
}
