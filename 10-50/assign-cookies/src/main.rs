extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn find_content_child(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let g_len: usize = g.len();
	let s_len: usize = s.len();
	if g_len == 0 || s_len == 0 {
	    return 0
	}

	let mut tmp_g: Vec<i32> = g.clone();
	qsi32::quick_sort(&mut tmp_g, 0, g_len - 1);
	let mut tmp_s: Vec<i32> = s.clone();
	qsi32::quick_sort(&mut tmp_s, 0, s_len - 1);
	let mut i: usize = 0;
	let mut result: i32 = 0;
        for n in tmp_g {
	    while i < s_len && tmp_s[i] < n {
	        i += 1;
	    }
	    if i == s_len {
	        break;
	    } else {
	        result += 1;
		i += 1;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut g: Vec<i32> = Vec::new();
    let mut s: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            n = i32::from_str(&arg).expect("Error parse.");
	} else if 1 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    if g.len() < n as usize {
	        g.push(number);
	    } else {
	        s.push(number);
	    }
	}
    }

    if 0 == ret || n as usize > ret {
        println!("Require at least (n + 1) parameters.");
	return;
    }

    println!("Childs: {}", Solution::find_content_child(g, s));
}
