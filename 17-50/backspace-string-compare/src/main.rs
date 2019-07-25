use std::env;

struct Solution {
}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut tmp_s: String = String::new();
	for c in s.chars() {
	    if c == '#' {
	        tmp_s.pop();
	    } else {
	        tmp_s.push(c);
	    }
	}
        let mut tmp_t: String = String::new();
	for c in t.chars() {
	    if c == '#' {
	        tmp_t.pop();
	    } else {
	        tmp_t.push(c);
	    }
	}
	tmp_s == tmp_t
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => s = arg,
	    _ => {
                ret += 1;
                let t: String = arg;
                println!("Same: {}", Solution::backspace_compare(s, t));
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

