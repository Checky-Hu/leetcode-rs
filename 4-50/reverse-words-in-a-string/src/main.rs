use std::env;

struct Solution {
}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result: String = String::new();
        let tmp_v: Vec<&str> = s.split_whitespace().collect();
	if tmp_v.is_empty() {
	    return result;
	}

	let mut i: usize = tmp_v.len() - 1;
	loop {
	    result.push_str(tmp_v[i]);
	    if i == 0 {
	        break;
	    } else {
	        result.push(' ');
	        i -= 1;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let s: String = arg;
            println!("String: {}", Solution::reverse_words(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
