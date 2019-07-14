use std::env;

struct Solution {
}

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut flags: Vec<i32> = vec![0; 26];
	let mut total: usize = 0;
	for c in license_plate.chars() {
	    if c >= 'A' && c <= 'Z' {
	        flags[(c as u8 - 65) as usize] += 1;
		total += 1;
	    } else if c >= 'a' && c <= 'z' {
	        flags[(c as u8 - 97) as usize] += 1;
		total += 1;
	    }
	}
	let mut result: String = String::new();
	for s in words {
	    if s.len() < total || (!result.is_empty() && s.len() >= result.len()) {
	        continue;
	    }

            let mut tmp: Vec<i32> = vec![0; 26];
	    for c in s.chars() {
	        if c >= 'a' && c <= 'z' {
	            tmp[(c as u8 - 97) as usize] += 1;
	        }
	    }
	    let mut i: usize = 0;
	    while i < 26 {
	        if flags[i] > tmp[i] {
		    break;
		} else {
		    i += 1;
		}
	    }
	    if i == 26 {
	        result = s;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut license_plate: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    1 => license_plate = arg,
	    _ => {
                ret += 1;
                let s: String = arg;
	        words.push(s);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Shortest completing word: {}", Solution::shortest_completing_word(license_plate, words));
}

