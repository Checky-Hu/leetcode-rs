use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_dist: i32 = 0;
	let mut is_zero: bool = true;
	let mut find_one: bool = false;
	let mut cur_zero_count: i32 = 0;
	for n in seats {
	    if n == 0 {
	        if is_zero {
	            cur_zero_count += 1;
		} else {
		    cur_zero_count = 1;
		    is_zero = true;
		}
	    } else {
	        // n == 1
	        if is_zero {
		    is_zero = false;
		    let tmp_dist: i32 = if find_one {
		        (cur_zero_count - 1) / 2 + 1
		    } else {
		        cur_zero_count
		    };
		    if tmp_dist > max_dist {
		        max_dist = tmp_dist;
		    }
		}
	        find_one = true;
	    }
	}
	if is_zero && cur_zero_count > max_dist {
	    max_dist = cur_zero_count;
	}
	max_dist
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut seats: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    seats.push(n);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Distance: {}", Solution::max_dist_to_closest(seats));
}

