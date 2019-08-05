use std::collections::HashSet;
use std::env;

struct Solution {
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut result: HashSet<String> = HashSet::new();
	for s in emails {
	    let mut tmp: String = String::new();
	    let mut reach_plus: bool = false;
	    let mut reach_at: bool = false;
	    for c in s.chars() {
	        match c {
	            '.' => {
		        if reach_at {
		            tmp.push(c);
		        }
		    },
		    '+' => {
		        reach_plus = true;
		        if reach_at {
		            tmp.push(c);
		        }
		    },
		    '@' => {
		        reach_at = true;
		        tmp.push(c);
		    },
		    _ => {
		        if reach_at || !reach_plus {
		            tmp.push(c);
		        }
		    },
	        }
	    }
	    if !result.contains(&tmp) {
	        result.insert(tmp);
	    }
	}
	result.len() as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut emails: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let s: String = arg;
	    emails.push(s);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Count: {}", Solution::num_unique_emails(emails));
}

