use std::env;

struct Solution {
}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
	if s.is_empty() {
	    return "".to_string()
	}

	let s_bytes: &[u8] = s.as_bytes();
        let mut result: Vec<u8> = Vec::new();
	let mut index: Vec<usize> = Vec::new();
	let mut i: usize = 0;
	while i < s_bytes.len() {
	    match s_bytes[i] as char {
		'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => {
		    result.push(0);
		    index.push(i);
		},
		_ => {
		    result.push(s_bytes[i]);
		},
	    }
	    i += 1;
	}

	let len: usize = index.len();
	if len != 0 {
	    i = 0;
	    while i <= len / 2 {
	        result[index[i]] = s_bytes[index[len - 1 - i]];
	        result[index[len - 1 - i]] = s_bytes[index[i]];
	        i += 1;
	    }
	}
	String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
	    let s: String = arg;
            println!("String: {}", Solution::reverse_vowels(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
