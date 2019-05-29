use std::env;

struct Solution {
}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let flags: Vec<i32> = vec![1, 2, 2, 1, 0, 1, 1,
	    1, 0, 1, 1, 1, 2, 2, 0, 0, 0, 0, 1, 0,
	    0, 2, 0, 2, 0, 2];
	for s in words {
	    let mut pre_c: i32 = -1;
	    let mut is_in_row: bool = true;
	    for c in s.chars() {
	        let i: usize = if c as u8 >= 97 {
		    (c as u8 - 97) as usize
		} else {
		    (c as u8 - 65) as usize
		};
		if pre_c == -1 {
		    pre_c = flags[i];
		} else {
		    if pre_c != flags[i] {
		        is_in_row = false;
		        break;
		    }
		}
	    }
	    if is_in_row {
	        result.push(s.clone());
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let s: String = arg;
	    words.push(s);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: Vec<String> = Solution::find_words(words);
    for s in result {
        println!("{}", s);
    }
}
