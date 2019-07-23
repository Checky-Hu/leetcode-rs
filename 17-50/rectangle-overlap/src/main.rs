use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let mut is_x_overlap: bool = false;
	if rec1[2] > rec2[0] {
	    if rec1[0] < rec2[2] {
	        is_x_overlap = true;
	    }
	}
	if !is_x_overlap {
	    return false
	}

        let mut is_y_overlap: bool = false;
	if rec1[3] > rec2[1] {
	    if rec1[1] < rec2[3] {
	        is_y_overlap = true;
	    }
	}
	is_y_overlap
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut rec1: Vec<i32> = Vec::new();
    let mut rec2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 <= index && index <= 4 {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    rec1.push(number);
	} else if 5 <= index && index <= 8 {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    rec2.push(number);
	}
	if ret == 8 {
	    break;
	}
    }

    if 8 != ret {
        println!("Require at least 8 parameters.");
	return;
    }

    println!("Is overlap: {}", Solution::is_rectangle_overlap(rec1, rec2));
}
