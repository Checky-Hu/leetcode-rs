use std::collections::HashSet;
use std::env;

struct Solution {
}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut result: HashSet<String> = HashSet::new();
        let flags: Vec<String> = vec![".-".to_string(), "-...".to_string(), "-.-.".to_string(), "-..".to_string(),
	    ".".to_string(), "..-.".to_string(), "--.".to_string(), "....".to_string(),
	    "..".to_string(), ".---".to_string(), "-.-".to_string(), ".-..".to_string(),
	    "--".to_string(), "-.".to_string(), "---".to_string(), ".--.".to_string(),
	    "--.-".to_string(), ".-.".to_string(), "...".to_string(), "-".to_string(),
	    "..-".to_string(), "...-".to_string(), ".--".to_string(), "-..-".to_string(),
	    "-.--".to_string(), "--..".to_string()];
	for word in words {
	    let mut s: String = String::new();
	    for c in word.chars() {
	        s.push_str(&flags[(c as u8 - 97) as usize]);
	    }
	    result.insert(s);
	}
	result.len() as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let s: String = arg;
	        words.push(s);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Count: {}", Solution::unique_morse_representations(words));
}

