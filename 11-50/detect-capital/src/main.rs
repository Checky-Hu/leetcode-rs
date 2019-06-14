use std::env;

struct Solution {
}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut pre_capital: bool = true;
	let mut i: usize = 0;
	for c in word.chars() {
	    let is_capital: bool = c as u8 >= 65 && c as u8 <= 90;
	    if pre_capital != is_capital {
		if i == 0 || (i == 1 && pre_capital) {
	            pre_capital = is_capital;
		} else {
		    return false
		}
	    }
	    i += 1;
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let word: String = arg;
            println!("Capital: {}", Solution::detect_capital_use(word));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
