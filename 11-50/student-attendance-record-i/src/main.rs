use std::env;

struct Solution {
}

impl Solution {
    pub fn check_record(s: String) -> bool {
	let mut a_count: i32 = 0;
        let mut l_count: i32 = 0;
	for c in s.chars() {
	    match c {
	        'A' => {
		    l_count = 0;
		    a_count += 1;
		    if a_count > 1 {
		        return false
		    }
		},
		'L' => {
		    l_count += 1;
		    if l_count > 2 {
		        return false
		    }
		},
		_ => l_count = 0,
	    }
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Reward: {}", Solution::check_record(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
