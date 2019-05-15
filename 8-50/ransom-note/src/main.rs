use std::env;

struct Solution {
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut r_count: Vec<i32> = vec![0; 26];
	let mut m_count: Vec<i32> = vec![0; 26];
	for c in ransom_note.chars() {
	    let index: usize = (c as u8 - 97) as usize;
	    r_count[index] += 1;
	}
	for c in magazine.chars() {
	    let index: usize = (c as u8 - 97) as usize;
	    m_count[index] += 1;
	}

	let mut i: usize = 0;
	while i < 26 {
	    if m_count[i] < r_count[i] {
	        return false
	    }
	    i += 1;
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut ransom_note: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => ransom_note = arg,
	    2 => {
                ret = index;
                let magazine: String = arg;
                println!("Include: {}", Solution::can_construct(ransom_note, magazine));
	        break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
