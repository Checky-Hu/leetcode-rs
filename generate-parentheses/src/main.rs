use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
	let mut i: i32 = 0;
	while i < 2 * n {
	    if i == 0 {
	        result.push('('.to_string());
	    } else {
	        let mut tmp_vec: Vec<String> = Vec::new();
		while !result.is_empty() {
		    tmp_vec.push(result.pop().unwrap());
		}
		for str in tmp_vec {
		    let mut open_parenthesis_count: i32 = 0;
		    let mut close_parenthesis_count: i32 = 0;
		    for c in str.chars() {
		        match c {
			    '(' => open_parenthesis_count += 1,
			    ')' => close_parenthesis_count += 1,
			    _ => (),
			}
		    }

		    if open_parenthesis_count != n {
		        let mut tmp_str: String = String::new();
			tmp_str.push_str(&str);
		        tmp_str.push('(');
		        result.push(tmp_str);
		    }

		    if open_parenthesis_count > close_parenthesis_count {
		        let mut tmp_str: String = String::new();
			tmp_str.push_str(&str);
			tmp_str.push(')');
		        result.push(tmp_str);
		    }
		}
	    }
	    i += 1;
	}

	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<String> = Solution::generate_parenthesis(n);
	    for str in result {
	        println!("{}", str);
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
