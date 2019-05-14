use std::env;

struct Solution {
}

impl Solution {
    pub fn first_unique_char(s: String) -> i32 {
        let mut indexes: Vec<i32> = vec![-1; 26];
	let mut count: Vec<i32> = vec![0; 26];
	let mut i: usize = 0;
	for c in s.chars() {
	    let index: usize = (c as u8 - 97) as usize;
	    count[index] += 1;
	    if indexes[index] == -1 {
	        indexes[index] = i as i32;
	    }
	    i += 1;
	}
	let mut result: i32 = -1;
	i = 0;
	while i < 26 {
	    if count[i] == 1 {
	        if result == -1 || indexes[i] < result {
		    result = indexes[i];
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
            let s: String = arg;
            println!("Index: {}", Solution::first_unique_char(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
