use std::env;

struct Solution {
}

impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let mut result: String = String::new();
	let mut tmp_c: char = ' ';
	let const_append: String = "ma".to_string();
	let mut tmp_append: String = "a".to_string();
	for c in s.chars() {
	    match c {
	        ' ' => {
		    if tmp_c != '\0' {
		        result.push(tmp_c);
		    }
		    tmp_c = ' ';
		    result.push_str(&const_append);
		    result.push_str(&tmp_append);
		    tmp_append.push('a');
		    result.push(' ');
		},
		'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
		    if tmp_c == ' ' {
		        tmp_c = '\0';
		    }
		    result.push(c);
		},
		_ => {
		    if tmp_c == ' ' {
		        tmp_c = c;
		    } else {
		        result.push(c);
		    }
		},
	    }
	}
	if tmp_c != '\0' {
	    result.push(tmp_c);
	}
	result.push_str(&const_append);
	result.push_str(&tmp_append);
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Goat: {}", Solution::to_goat_latin(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

