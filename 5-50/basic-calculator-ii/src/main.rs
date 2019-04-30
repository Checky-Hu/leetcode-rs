use std::env;

struct Solution {
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut num: i32 = 0;
	let mut cur_res: i32 = 0;
	let mut result: i32 = 0;
	let mut operator: char = '+';
	for c in s.chars() {
	    match c {
	        ' ' => (),
	        '*' | '/' | '+' | '-' => {
		    match operator {
		        '+' => cur_res += num,
			'-' => cur_res -= num,
			'*' => cur_res *= num,
			'/' => cur_res /= num,
		        _ => (),
		    }
		    if c == '+' || c == '-' {
		        result += cur_res;
			cur_res = 0;
		    }
		    num = 0;
		    operator = c;
		},
		_ => {
		    num = num * 10 + (c as u8 - 48) as i32;
		},
	    }
	}

	result + match operator {
	    '+' => cur_res + num,
	    '-' => cur_res - num,
	    '*' => cur_res * num,
	    '/' => cur_res / num,
	    _ => result,
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Result: {}", Solution::calculate(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
