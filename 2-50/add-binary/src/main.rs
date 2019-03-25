use std::env;

struct Solution {
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result: String = String::new();

	let a_bytes: &[u8] = a.as_bytes();
	let a_len: usize = a.len();
	let b_bytes: &[u8] = b.as_bytes();
	let b_len: usize = b.len();
	let mut offset: usize = 1;
	let mut carry_bit: u8 = 0;
	while offset <= a_len {
	    let tmp_sum: u8 = carry_bit + if offset <= b_len {
	        a_bytes[a_len - offset] + b_bytes[b_len - offset] - 96
	    } else {
		a_bytes[a_len - offset] - 48
	    };
	    result.insert(0, (tmp_sum % 2 + 48) as char);
	    carry_bit = tmp_sum / 2;
	    offset += 1;
	}
	while offset <= b_len {
	    let tmp_sum: u8 = carry_bit + b_bytes[b_len - offset] - 48;
	    result.insert(0, (tmp_sum % 2 + 48) as char);
	    carry_bit = tmp_sum / 2;
	    offset += 1;
	}
	if carry_bit > 0 {
	    result.insert(0, '1');
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: String = String::new();
    let mut b: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
	    a = arg;
	} else if 2 == index {
            ret = index;
	    b = arg;
	    break;
	} else {
	    continue;
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    println!("String: {}", Solution::add_binary(a, b));
}
