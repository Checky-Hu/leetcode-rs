use std::collections::HashMap;
use std::env;

struct Solution {
}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
	let mut map: Vec<String> = vec![" ".to_string(); 26];
	let mut table: HashMap<String, char> = HashMap::new();
	let strs: Vec<&str> = str.split_whitespace().collect();
	let mut i: usize = 0;
	for c in pattern.chars() {
	    if i >= strs.len() {
	        return false
	    }

	    let index: usize = (c as u8 - 97) as usize;
	    if map[index] == " " {
	        let tmp_s: String = strs[i].to_string();
		if table.contains_key(&tmp_s) {
		    return false
		} else {
		    table.insert(tmp_s.clone(), c);
		    map[index] = tmp_s;
		    i += 1;
		}
	    } else if map[index] == strs[i] {
	        i += 1;
	    } else {
	        return false
	    }
	}
	if i == strs.len() {
	    true
	} else {
	    false
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut pattern: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => pattern = arg,
	    2 => {
                ret = index;
                let str: String = arg;
                println!("Match: {}", Solution::word_pattern(pattern, str));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
