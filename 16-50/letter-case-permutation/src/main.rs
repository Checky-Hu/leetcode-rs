use std::env;

struct Solution {
}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
	result.push(s.clone());
	let mut i: usize = 0;
	for c in s.chars() {
	    let mut is_character: bool = false;
	    let mut is_lower: bool = false;
	    if c >= 'A' && c <= 'Z' {
	        is_character = true;
	    } else if c >= 'a' && c <= 'z' {
	        is_character = true;
		is_lower = true;
	    }

	    if is_character {
	        let mut tmp_v: Vec<String> = Vec::new();
	        while !result.is_empty() {
		    tmp_v.push(result.pop().unwrap());
		}

	        for tmp_s in tmp_v {
		    let mut cur_s: String = String::new();
		    let mut j: usize = 0;
		    for tmp_c in tmp_s.chars() {
		        if j == i {
		            cur_s.push(if is_lower {
			        (tmp_c as u8 - 32) as char
			    } else {
			        (tmp_c as u8 + 32) as char
			    });
			} else {
		            cur_s.push(tmp_c);
			}
			j += 1;
		    }
		    result.push(cur_s);
		    result.push(tmp_s);
		}
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    ret = 1;
	    let s: String = arg;
	    let result: Vec<String> = Solution::letter_case_permutation(s);
	    for tmp_s in result {
                println!("{} ", tmp_s);
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
