use std::env;

struct Solution {
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_count: Vec<i32> = vec![0; 26];
	for c in s.chars() {
	    s_count[(c as u8 - 97) as usize] += 1;
	}
        let mut t_count: Vec<i32> = vec![0; 26];
	for c in t.chars() {
	    t_count[(c as u8 - 97) as usize] += 1;
	}
	let mut i: usize = 0;
	while i < 26 {
	    if s_count[i] != t_count[i] {
	        return false
	    }
	    i += 1;
	}
	true
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
		println!("Anagram: {}", Solution::is_anagram(s, t));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
