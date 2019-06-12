use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 0 {
	    return false
	}

        let mut sum: i32 = 0;
	let mut i: i32 = (num as f64).sqrt() as i32;
	while i > 1 {
	    if num % i == 0 {
	        sum += num / i;
		sum += i;
	    }
	    i -= 1;
	}
	if num != 1 {
	    sum += 1;
	}
	sum == num
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Perfect number: {}", Solution::check_perfect_number(num));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
