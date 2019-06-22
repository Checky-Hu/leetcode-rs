use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let len1: usize = list1.len();
        let len2: usize = list2.len();
	let mut end1: usize = len1 - 1;
	let mut min_sum_i: usize = end1 + len2 - 1;
	let mut i: usize = 0;
	while i <= end1 {
	    let mut j: usize = 0;
	    let end2: usize = if min_sum_i - i < len2 - 1 {
	        min_sum_i - i
	    } else {
	        len2 - 1
	    };
	    while j <= end2 {
	        if list1[i] == list2[j] {
		    break;
		} else {
		    j += 1;
		}
	    }
	    if j <= end2 {
	        if i + j == min_sum_i {
		    result.push(list1[i].clone());
		} else if i + j < min_sum_i {
		    min_sum_i = i + j;
		    result.clear();
		    result.push(list1[i].clone());
		}
		if min_sum_i < end1 {
		    end1 = min_sum_i;
		}
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut len: i32 = 0;
    let mut list1: Vec<String> = Vec::new();
    let mut list2: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => len = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
		if list1.len() < len as usize {
	            list1.push(arg);
		} else {
	            list2.push(arg);
		}
	    },
	}
    }

    if 0 == ret || len >= ret {
        println!("Require at least (arg1 + 1) parameters.");
	return;
    }

    let result: Vec<String> = Solution::find_restaurant(list1, list2);
    for s in result {
	println!("{}", s);
    }
}
