use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut i: usize = 1;
	let mut increasing: i32 = 0;
	while i < a.len() {
	    if a[i] > a[i - 1] {
	        if increasing == 0 {
		    increasing = 1;
		} else if increasing == -1 {
		    return false
		}
	    } else if a[i] < a[i - 1] {
	        if increasing == 0 {
		    increasing = -1;
		} else if increasing == 1 {
		    return false
		}
	    }
	    i += 1;
	}
	true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    a.push(n);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Monotonic: {}", Solution::is_monotonic(a));
}

