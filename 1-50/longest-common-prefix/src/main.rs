use std::env;

struct Solution {
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result: String = String::new();
	let mut count: usize = 0;
        loop {
	    let mut cur_c: char = '0';
	    let mut index: usize = 0;
            for str in &strs {
	        match index {
		    0 => {
	                cur_c = match str.chars().nth(count) {
	                    Some(c) => c,
		            None => '0',
		        };
		    },
		    _ => {
	                match str.chars().nth(count) {
		            Some(c) => {
			        if cur_c != c {
				    break;
				}
			    },
		            None => break,
		        }
		    },
	        };
		index += 1;
	    }

	    if strs.len() == index && cur_c != '0' {
	        result.push(cur_c);
		count += 1;
	    } else {
	        break;
	    }
	}

	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut strs: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if index > 0 {
	    ret = index;
	    strs.push(arg);
	}
    }
    println!("String: {}", Solution::longest_common_prefix(strs));

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
