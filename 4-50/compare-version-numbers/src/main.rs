use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1_strs: Vec<&str> = version1.split('.').collect();
	let len1: usize = v1_strs.len();
        let v2_strs: Vec<&str> = version2.split('.').collect();
	let len2: usize = v2_strs.len();
	let mut i: usize = 0;
	while i < len1 && i < len2 {
	    let v1_n: i32 = i32::from_str(v1_strs[i]).expect("Parse error.");
	    let v2_n: i32 = i32::from_str(v2_strs[i]).expect("Parse error.");
	    if v1_n > v2_n {
	        return 1
	    } else if v1_n < v2_n {
	        return -1
	    } else {
	        i += 1;
	    }
	}

	if len1 < len2 {
	    while i < len2 {
	        let v_n: i32 = i32::from_str(v2_strs[i]).expect("Parse error.");
		if v_n > 0 {
		    return -1
		} else {
		    i += 1;
		}
	    }
	    0
	} else if len1 > len2 {
	    while i < len1 {
	        let v_n: i32 = i32::from_str(v1_strs[i]).expect("Parse error.");
		if v_n > 0 {
		    return 1
		} else {
		    i += 1;
		}
	    }
	    0
	} else {
	    0
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut version1: String = String::new();
    let mut version2: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => version1 = arg,
	    2 => {
                ret = index;
		version2 = arg;
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    println!("Result: {}", Solution::compare_version(version1, version2));
}
