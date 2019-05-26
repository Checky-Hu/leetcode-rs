use std::env;

struct Solution {
}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let len: usize = chars.len();
        let mut i: usize = 0;
	let mut j: usize = 0;
	let mut pre_c: char = chars[0];
	let mut count: i32 = 0;
	while j < len {
	    if pre_c == chars[j] {
	        count += 1;
	    } else {
	        chars[i] = pre_c;
		i += 1;
		if count > 1 {
		    let mut tmp: i32 = 10;
		    while tmp <= count {
		        tmp *= 10;
		    }
		    tmp /= 10;
		    while count >= 10 {
		        let res: i32 = count / tmp;
		        chars[i] = (res as u8 + 48) as char;
		        i += 1;
			count = count - res * tmp;
		    }
		    chars[i] = (count as u8 + 48) as char;
		    i += 1;
		}
		pre_c = chars[j];
		count = 1;
	    }
	    j += 1;
	}
	chars[i] = pre_c;
	i += 1;
	if count > 1 {
	    let mut tmp: i32 = 10;
	    while tmp <= count {
		tmp *= 10;
	    }
	    tmp /= 10;
	    while count >= 10 {
		let res: i32 = count / tmp;
		chars[i] = (res as u8 + 48) as char;
		i += 1;
		count = count - res * tmp;
	    }
	    chars[i] = (count as u8 + 48) as char;
	    i += 1;
	}
	i as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut chars: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let c: char = arg.chars().next().unwrap();
	    chars.push(c);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: i32 = Solution::compress(&mut chars);
    let mut i: i32 = 0;
    while i < result {
        print!("{} ", chars[i as usize]);
	i += 1;
    }
    print!("\nLength: {}\n", result);
}
