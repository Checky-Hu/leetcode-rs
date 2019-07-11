use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut n: i32 = left;
	while n <= right {
	    let mut tmp: i32 = n;
	    while tmp > 0 {
	        let cur_n: i32 = tmp % 10;
		if cur_n == 0 {
		    break;
		} else if n % cur_n == 0 {
		    tmp /= 10;
		} else {
		    break;
		}
	    }
	    if tmp == 0 {
	        result.push(n);
	    }
	    n += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut left: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    left = i32::from_str(&arg).expect("Error parse.");
	} else if 2 == index {
            ret += 1;
            let right: i32 = i32::from_str(&arg).expect("Error parse.");
	    let result: Vec<i32> = Solution::self_dividing_numbers(left, right);
	    for n in result {
                print!("{} ", n);
	    }
	    print!("\n");
	    break;
	} else {
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}

