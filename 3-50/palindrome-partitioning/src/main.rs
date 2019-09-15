use std::env;

struct Solution {
}

impl Solution {
    pub fn is_palindrome(s_bytes: &[u8], left: usize, right: usize) -> bool {
        if left >= right {
	    return true
	}

	let mut i: usize = left;
	let mut j: usize = right;
        while i < j {
	    if s_bytes[i] != s_bytes[j] {
	        return false
	    } else {
	        i += 1;
		j -= 1;
	    }
	}
	true
    }

    pub fn get_next_partition(s_bytes: &[u8], left: usize, right: usize, mut v: &mut Vec<String>, mut result: &mut Vec<Vec<String>>) {
        if left > right {
	    result.push(v.to_vec());
	    return
	}
	for i in left..=right {
	    if Solution::is_palindrome(s_bytes, left, i) {
	        let mut tmp: String = String::new();
		for j in left..=i {
		    tmp.push(s_bytes[j] as char);
		}
	        v.push(tmp);
		Solution::get_next_partition(s_bytes, i + 1, right, &mut v, &mut result);
		v.pop();
	    }
	}
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut v: Vec<String> = Vec::new();
	let s_bytes: &[u8] = s.as_bytes();
	let len: usize = s_bytes.len();
	Solution::get_next_partition(s_bytes, 0, len - 1, &mut v, &mut result);
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let s: String = arg;
	    let result: Vec<Vec<String>> = Solution::partition(s);
	    for v in result {
	        for t in v {
	            print!("{} ", t);
		}
		print!("\n");
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
