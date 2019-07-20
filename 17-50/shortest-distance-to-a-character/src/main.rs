use std::env;

struct Solution {
}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s_bytes: &[u8] = s.as_bytes();
	let s_len: usize = s_bytes.len();
        let mut result: Vec<i32> = Vec::new();
	let mut pre_c: i32 = -1;
	loop {
	    let mut i: i32 = pre_c + 1;
	    while (i as usize) < s_len {
	        if s_bytes[i as usize] == (c as u8) {
		    break;
		} else {
		    i += 1;
		}
	    }
	    if i as usize == s_len {
	        i = pre_c + 1;
		while (i as usize) < s_len {
		    result.push(i - pre_c);
		    i += 1;
		}
		break;
	    } else {
	        if pre_c < 0 {
		    for j in 0..=i {
		        result.push(i - j);
		    }
		} else {
	            let mid: i32 = pre_c + (i - pre_c) / 2;
		    let mut j: i32 = pre_c + 1;
		    while j <= mid {
		        result.push(j - pre_c);
			j += 1;
		    }
		    while j <= i {
		        result.push(i - j);
			j += 1;
		    }
		}
		pre_c = i;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => s = arg,
	    2 => {
                ret += 1;
                let c: char = arg.chars().next().unwrap();
		let result: Vec<i32> = Solution::shortest_to_char(s, c);
		for i in result {
                    print!("{} ", i);
		}
		print!("\n");
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
