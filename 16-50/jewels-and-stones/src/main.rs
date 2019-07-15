use std::env;

struct Solution {
}

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
	let mut flags: Vec<bool> = vec![false; 52];
	for c in j.chars() {
	    if c >= 'A' && c <= 'Z' {
	        flags[(c as u8 - 65) as usize] = true;
	    } else {
	        flags[(c as u8 - 71) as usize] = true;
	    }
	}

        let mut result: i32 = 0;
	for c in s.chars() {
	    if c >= 'A' && c <= 'Z' {
	        if flags[(c as u8 - 65) as usize] {
		    result += 1;
		}
	    } else {
	        if flags[(c as u8 - 71) as usize] {
		    result += 1;
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut j: String = String::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => j = arg,
	    _ => {
                ret += 1;
                let s: String = arg;
                println!("Count: {}", Solution::num_jewels_in_stones(j, s));
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

