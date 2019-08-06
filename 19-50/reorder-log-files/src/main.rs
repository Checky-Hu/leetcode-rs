use std::env;

struct Solution {
}

impl Solution {
    pub fn is_digits(s: &String) -> bool {
	let mut reach_id: bool = false;
        for c in s.chars() {
            if reach_id {
	        if c >= '0' && c <= '9' {
		    return true
		} else {
		    return false
		}
	    } else {
	        if c == ' ' {
		    reach_id = true;
		}
	    }
	}
	false
    }

    pub fn compare(a: &String, b: &String) -> bool {
        let a_bytes: &[u8] = a.as_bytes();
	let a_len: usize = a_bytes.len();
        let b_bytes: &[u8] = b.as_bytes();
	let b_len: usize = b_bytes.len();
	let mut a_pos: usize = 0;
	for c in a.chars() {
	    a_pos += 1;
	    if c == ' ' {
	        break;
	    }
	}
	let mut b_pos: usize = 0;
	for c in b.chars() {
	    b_pos += 1;
	    if c == ' ' {
	        break;
	    }
	}
	let mut i: usize = a_pos;
	let mut j: usize = b_pos;
	while i < a_len {
	    if j == b_len {
	        return true
	    } else {
	        if a_bytes[i] > b_bytes[j] {
		    return true
		} else if a_bytes[i] < b_bytes[j] {
		    return false
		} else {
		    i += 1;
		    j += 1;
		}
	    }
	}
	if j < b_len {
	    return false
	}
	i = 0;
	j = 0;
	while i < a_pos - 1 {
	    if j == b_pos - 1 {
	        return true
	    } else {
	        if a_bytes[i] > b_bytes[j] {
		    return true
		} else if a_bytes[i] < b_bytes[j] {
		    return false
		} else {
		    i += 1;
		    j += 1;
		}
	    }
	}
	false
    }

    pub fn quick_sort(s_v: &mut Vec<String>, left: usize, right: usize) {
        if left >= right {
	    return;
	}

	let mut i: usize = left;
	let mut j: usize = right;
	let temp: String = s_v[left].clone();
	while i < j {
	    while i < j && Solution::compare(&s_v[j], &temp) {
	        j -= 1;
	    }
	    s_v[i] = s_v[j].clone();
	    while i < j && !Solution::compare(&s_v[i], &temp) {
	        i += 1;
	    }
	    s_v[j] = s_v[i].clone();
	}
	s_v[i] = temp;
	if left + 1 < i {
	    Solution::quick_sort(s_v, left, i - 1);
	}
	Solution::quick_sort(s_v, i + 1, right);
    }

    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
	let len: usize = logs.len();
	if len == 0 {
	    return Vec::new()
	}

        let mut result: Vec<String> = logs.clone();
	let mut i: usize = len - 1;
	let mut digits_pos: usize = i;
	while i > 0 {
	    if Solution::is_digits(&result[i]) {
	        digits_pos = i;
	    } else {
	        let mut found_digits: bool = false;
	        let mut j: usize = digits_pos - 1;
	        loop {
		    if Solution::is_digits(&result[j]) {
		        found_digits = true;
		        break;
		    } else {
		        if j == 0 {
			    break;
			} else {
		            j -= 1;
			}
		    }
		}
		if found_digits {
		    digits_pos = i;
		    let tmp: String = result[i].clone();
		    result[i] = result[j].clone();
		    result[j] = tmp;
		}
	    }
	    i -= 1;
	}
	if digits_pos > 1 {
	    Solution::quick_sort(&mut result, 0, digits_pos - 1);
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut logs: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let s: String = arg;
	    logs.push(s);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: Vec<String> = Solution::reorder_log_files(logs);
    for s in result {
        println!("{}", s);
    }
}
