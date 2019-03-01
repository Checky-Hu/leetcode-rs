use std::env;

struct Solution {
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
	let mut count: usize = 0;
        let mut vector: Vec<char> = Vec::new();
	for c in s.chars() {
	    match c {
	        '(' | '[' | '{' => vector.push(c),
		')' | ']' | '}' => {
		    let _target: char = match c {
		        ')' => '(',
			']' => '[',
			'}' => '{',
			_ => '0',
		    };
		    match vector.pop() {
		        Some(prefix_c) => {
			    if prefix_c != _target {
				break;
			    }
			},
			None => break,
		    }
		},
		_ => continue,
	    }
	    count += 1;
	}

	if vector.is_empty() && count == s.len() {
	    true
	} else {
	    false
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut str: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
	    ret = index;
	    str = arg;
	    break;
	}
    }
    println!("Valid: {}", Solution::is_valid(str));

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
