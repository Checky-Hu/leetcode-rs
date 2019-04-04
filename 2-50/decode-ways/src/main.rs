use std::env;

struct Solution {
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
	let mut result: Vec<i32> = vec![1];
	let mut index: usize = 1;
	let mut preceding_c: char = '-';
	for c in s.chars() {
	    let tmp_i: i32 = if c == '0' {
	        match preceding_c {
		    '1' | '2' => result[index - 2],
		    _ => 0,
		}
	    } else {
		match preceding_c {
		    '-' => 1,
		    '1' => result[index - 1] + result[index - 2],
		    '2' => {
		        if c > '6' {
		            result[index - 1]
		        } else {
		            result[index - 1] + result[index - 2]
		        }
		    },
		    _ => result[index - 1],
		}
	    };
	    preceding_c = c;
	    result.push(tmp_i);
	    index += 1;
	}
	result[index - 1]
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
	    let s: String = arg;
            println!("Ways: {}", Solution::num_decodings(s));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
