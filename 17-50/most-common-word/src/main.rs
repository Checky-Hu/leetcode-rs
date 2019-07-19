use std::collections::HashMap;
use std::env;

struct Solution {
}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut map: HashMap<String, i32> = HashMap::new();
	let mut is_alpha: bool = false;
	let mut tmp_s: String = String::new();
	for c in paragraph.chars() {
	    if c >= 'A' && c <= 'Z' {
	        tmp_s.push((c as u8 + 32) as char);
		is_alpha = true;
	    } else if c >= 'a' && c <= 'z' {
	        tmp_s.push(c);
		is_alpha = true;
	    } else {
	        if is_alpha {
		    if map.contains_key(&tmp_s) {
		        *(map.get_mut(&tmp_s).unwrap()) += 1;
		    } else {
		        map.insert(tmp_s, 1);
		    }
		    is_alpha = false;
		    tmp_s = String::new();
		}
	    }
	}
	if is_alpha {
	    if map.contains_key(&tmp_s) {
		*(map.get_mut(&tmp_s).unwrap()) += 1;
	    } else {
		map.insert(tmp_s, 1);
	    }
	}

	let mut max_count: i32 = 0;
	let mut result: String = String::new();
	for (key, value) in map.iter() {
	    if *value > max_count {
	        let mut is_banned: bool = false;
		for s in &banned {
		    if *s == *key {
		        is_banned = true;
			break;
		    }
		}
		if !is_banned {
	            max_count = *value;
		    result = key.to_string();
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut paragraph: String = String::new();
    let mut banned: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => paragraph = arg,
	    _ => {
	        ret += 1;
                let s: String = arg;
		banned.push(s);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
	return;
    }

    println!("Word: {}", Solution::most_common_word(paragraph, banned));
}
