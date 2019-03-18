use std::env;

struct Solution {
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes: &[u8] = s.as_bytes();
	let mut s_index: usize = 0;
        let p_bytes: &[u8] = p.as_bytes();
	let mut p_index: usize = 0;

	let mut wildchar_pos: i32 = -1;
	while s_index < s.len() {
	    let tmp_byte: u8 = if p_index >= p.len() {
	        0
	    } else {
	        p_bytes[p_index]
	    };

	    if tmp_byte == '?' as u8 || tmp_byte == s_bytes[s_index] {
	        s_index += 1;
		p_index += 1;
	    } else if tmp_byte == '*' as u8 {
		wildchar_pos = p_index as i32 + 1;
		p_index += 1;
	    } else {
		if wildchar_pos > 0 {
		    s_index = s_index - (p_index - wildchar_pos as usize) + 1;
		    p_index = wildchar_pos as usize;
		} else {
		    return false
		}
	    }
	}

	while p_index < p.len() {
	    if p_bytes[p_index] == '*' as u8 {
	        p_index += 1;
	    } else {
	        return false
	    }
	}
        true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    let mut p: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
	    s = arg;
	} else if 2 == index {
            ret = index;
            p = arg;
	    break;
	} else {
	    continue;
	}
    }

    if 2 != ret {
        println!("Require at least two parameter.");
	return;
    }

    println!("Match: {}", Solution::is_match(s, p));
}
