use std::env;

struct Solution {
}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s_count: Vec<i32> = vec![0; 26];
	let mut t_count: Vec<i32> = vec![0; 26];
	for c in s.chars() {
	    let index: usize = (c as u8 - 97) as usize;
	    s_count[index] += 1;
	}
	for c in t.chars() {
	    let index: usize = (c as u8 - 97) as usize;
	    t_count[index] += 1;
	}

	let mut i: usize = 0;
	while i < 26 {
	    if s_count[i] < t_count[i] {
	        return (i as u8 + 97) as char
	    }
	    i += 1;
	}
	'\0'
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => s = arg,
	    2 => {
                ret = index;
                let t: String = arg;
                println!("Char: {}", Solution::find_the_difference(s, t));
	        break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
