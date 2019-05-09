use std::env;

struct Solution {
}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len: usize = s.len();
	let mut i: usize = 0;
	while i < len / 2 {
	    let tmp: char = s[i];
	    s[i] = s[len - 1 - i];
	    s[len - 1 - i] = tmp;
	    i += 1;
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let c: char = arg.chars().next().unwrap();
	    s.push(c);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    Solution::reverse_string(&mut s);
    for c in s {
        print!("{} ", c);
    }
    print!("\n");
}
