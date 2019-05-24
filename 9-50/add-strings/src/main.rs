use std::env;

struct Solution {
}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1_bytes: &[u8] = num1.as_bytes();
	let len1: usize = num1_bytes.len();
        let num2_bytes: &[u8] = num2.as_bytes();
	let len2: usize = num2_bytes.len();
	let (longer, shorter, is_num1) = if len1 > len2 {
	    (len1, len2, true)
	} else {
	    (len2, len1, false)
	};
	let mut result: Vec<u8> = vec![0; longer + 1];
	let mut i: usize = 1;
	let mut carry: u8 = 0;
	while i <= longer {
	    let tmp: u8 = if i <= shorter {
	        carry + num1_bytes[len1 - i] + num2_bytes[len2 - i] - 96
	    } else {
	        carry + if is_num1 {
		    num1_bytes[len1 - i]
		} else {
		    num2_bytes[len2 - i]
		} - 48
	    };
	    if tmp >= 10 {
	        carry = 1;
		result[longer + 1 - i] = tmp + 38;
	    } else {
	        carry = 0;
		result[longer + 1 - i] = tmp + 48;
	    }
	    i += 1;
	}
	if carry == 1 {
	    result[0] = 49;
	} else {
	    result.remove(0);
	}
	String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut num1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => num1 = arg,
	    2 => {
                ret = index;
                let num2: String = arg;
                println!("String: {}", Solution::add_strings(num1, num2));
	        break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
