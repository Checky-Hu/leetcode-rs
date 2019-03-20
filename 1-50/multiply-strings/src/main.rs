use std::env;

struct Solution {
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result: String = String::new();
	let mut carry_bit: u8 = 0;

	let num1_bytes: &[u8] = num1.as_bytes();
	let num1_len: usize = num1_bytes.len();
	let num2_bytes: &[u8] = num2.as_bytes();
	let num2_len: usize = num2_bytes.len();
	let mut i: usize = num1_len - 1;
	while i >= 0 {
	    let mut j: usize = num2_len - 1;
	}
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut num1: String = String::new();
    let mut num2: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => num1 = arg,
	    2 => {
                ret = index;
                num2 = arg;
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    let result: String = Solution::multiply(num1, num2);
    println!("String: {}", result);
}
