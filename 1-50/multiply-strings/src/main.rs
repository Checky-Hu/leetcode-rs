use std::env;

struct Solution {
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut res_str: String = String::new();

	let num1_bytes: &[u8] = num1.as_bytes();
	let num1_len: usize = num1_bytes.len();
	let num2_bytes: &[u8] = num2.as_bytes();
	let num2_len: usize = num2_bytes.len();
	if num1_bytes[0] == 48 || num2_bytes[0] == 48 {
	    return "0".to_string()
	}

	let mut i: usize = num1_len - 1;
	let mut offset: usize = 0;
	loop {
	    let mut tmp_str: String = String::new();
	    let mut offset_count: usize = 0;
	    while offset_count < offset {
	        tmp_str.push('0');
		offset_count += 1;
	    }

	    let mut j: usize = num2_len - 1;
	    let mut carry_bit: u8 = 0;
	    loop {
	        let tmp_res: u8 = carry_bit + (num1_bytes[i] - 48 ) * (num2_bytes[j] - 48);
		tmp_str.push((tmp_res % 10 + 48) as char);
		carry_bit = tmp_res / 10;
		if j >= 1 {
		    j -= 1;
		} else {
		    break;
		}
	    }
	    if carry_bit > 0 {
	        tmp_str.push((carry_bit + 48) as char);
		carry_bit = 0;
	    }

	    let tmp_bytes: &[u8] = tmp_str.as_bytes();
	    let preceding_str: String = res_str.clone();
	    res_str.clear();
	    let preceding_bytes: &[u8] = preceding_str.as_bytes();
	    let mut index: usize = 0;
	    while index < tmp_bytes.len() {
	        let tmp_res: u8 = if index < preceding_bytes.len() {
		    carry_bit + tmp_bytes[index] + preceding_bytes[index] - 96
		} else {
		    carry_bit + tmp_bytes[index] - 48
		};
		res_str.push((tmp_res % 10 + 48) as char);
		carry_bit = tmp_res / 10;
		index += 1;
	    }
	    if carry_bit > 0 {
	        res_str.push((carry_bit + 48) as char);
	    }

	    if i >= 1 {
	        i -= 1;
	        offset += 1;
	    } else {
	        break;
	    }
	}

	let mut result: String = String::new();
	while !res_str.is_empty() {
	    result.push(res_str.pop().unwrap());
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

    if 2 != ret {
        println!("Require at least two parameter.");
	return;
    }

    let result: String = Solution::multiply(num1, num2);
    println!("String: {}", result);
}
