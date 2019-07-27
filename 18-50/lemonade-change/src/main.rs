use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
	let mut five_count: i32 = 0;
	let mut ten_count: i32 = 0;
	for c in bills {
	    if 5 == c {
		five_count += 1;
	    } else if 10 == c {
	        if five_count == 0 {
		    return false
		} else {
		    five_count -= 1;
		    ten_count += 1;
		}
	    } else {
	        if five_count == 0 {
		    return false
		} else {
		    if ten_count > 0 {
		        five_count -= 1;
		        ten_count -= 1;
		    } else {
		        if five_count >= 3 {
			    five_count -= 3;
			} else {
			    return false
			}
		    }
		}
	    }
	}
	true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut bills: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    bills.push(n);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Change: {}", Solution::lemonade_change(bills));
}

