use std::env;

struct Solution {
}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
	let s_bytes: &[u8] = s.as_bytes();
	let s_len: usize = s_bytes.len();
	let p_bytes: &[u8] = p.as_bytes();
	let p_len: usize = p_bytes.len();
	if s_len < p_len {
	    return result
	}

	let mut flags: Vec<i32> = vec![0; 26];
	let mut count: Vec<i32> = vec![0; 26];
	let mut i: usize = 0;
	while i < p_len {
	    flags[p_bytes[i] as usize - 97] += 1;
	    count[s_bytes[i] as usize - 97] += 1;
	    i += 1;
	}

	i = 0;
	let mut is_anagram: bool = false;
	while i <= s_len - p_len {
	    if is_anagram {
	        if s_bytes[i - 1] == s_bytes[i + p_len - 1] {
		    result.push(i as i32);
		} else {
		    count[s_bytes[i - 1] as usize - 97] -= 1;
		    count[s_bytes[i + p_len - 1] as usize - 97] += 1;
		    is_anagram = false;
		}
	    } else {
	        if i > 0 {
		    count[s_bytes[i - 1] as usize - 97] -= 1;
		    count[s_bytes[i + p_len - 1] as usize - 97] += 1;
		}
		let mut j: usize = 0;
		while j < 26 {
		    if flags[j] != count[j] {
		        break;
		    } else {
		        j += 1;
		    }
		}
		if j == 26 {
		    result.push(i as i32);
		    is_anagram = true;
		}
	    }
	    i += 1;
	}
	result
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
                let p: String = arg;
		let result: Vec<i32> = Solution::find_anagrams(s, p);
		for n in result {
                    print!("{} ", n);
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
