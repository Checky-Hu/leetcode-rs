use std::env;

struct Solution {
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len: usize = s.len();
	if len == 0 {
	    return "".to_string()
	}
	let src_bytes: &[u8] = s.as_bytes();

	let mut index: usize = 0;
	let mut substr_index: usize = 0;
	let mut substr_len: usize = 1;
	while index + substr_len + 1 <= len {
	    let tmp_index: usize = index;
	    let tmp_max_len: usize = len - tmp_index;
	    let mut cur_try_len: usize = tmp_max_len;
	    while cur_try_len > 1 {
	        let mut cur_offset: usize = 0;
		let cur_max_offset: usize = cur_try_len / 2 - 1;
		while cur_offset <= cur_max_offset {
	            if src_bytes[tmp_index + cur_offset] != src_bytes[tmp_index + cur_try_len - cur_offset - 1] {
		        break;
		    }
		    cur_offset += 1;
		}

		if cur_offset > cur_max_offset {
		    break;
		} else {
	            cur_try_len -= 1;
		}
	    }

	    if cur_try_len > substr_len {
	        substr_len = cur_try_len;
		substr_index = index;
	    }
	    index +=1;
	}

	let mut result: String = String::new();
	let mut res_len: usize = 0;
	while res_len < substr_len {
	    result.push(src_bytes[substr_index + res_len] as char);
	    res_len += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            println!("String: {}", Solution::longest_palindrome(arg));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
