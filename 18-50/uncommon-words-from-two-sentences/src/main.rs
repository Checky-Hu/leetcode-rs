use std::collections::HashMap;
use std::env;

struct Solution {
}

impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let a_strs: Vec<&str> = a.split_whitespace().collect();
	let mut a_map: HashMap<String, i32> = HashMap::new();
	for tmp in a_strs {
	    if a_map.contains_key(tmp) {
	        *a_map.get_mut(tmp).unwrap() += 1;
	    } else {
	        a_map.insert(tmp.to_string(), 1);
	    }
	}
        let b_strs: Vec<&str> = b.split_whitespace().collect();
	let mut b_map: HashMap<String, i32> = HashMap::new();
	for tmp in b_strs {
	    if b_map.contains_key(tmp) {
	        *b_map.get_mut(tmp).unwrap() += 1;
	    } else {
	        b_map.insert(tmp.to_string(), 1);
	    }
	}
	for (key, val) in a_map.iter() {
	    if *val == 1 && !b_map.contains_key(key) {
	        result.push(key.to_string());
	    }
	}
	for (key, val) in b_map.iter() {
	    if *val == 1 && !a_map.contains_key(key) {
	        result.push(key.to_string());
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: String = String::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => a = arg,
	    _ => {
                ret += 1;
                let b: String = arg;
		let result: Vec<String> = Solution::uncommon_from_sentences(a, b);
		for s in result {
                    println!("{}", s);
		}
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

