use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
	let mut i: usize = digits.len() - 1;
	let mut carry_bit: i32 = 0;
	loop {
	    let tmp_sum: i32 = if i == digits.len() - 1 {
	        digits[i] + 1
	    } else {
	        digits[i] + carry_bit
	    };
	    result.insert(0, tmp_sum % 10);
	    carry_bit = tmp_sum / 10;
	    if i >= 1 {
	        i -= 1;
	    } else {
	        break;
	    }
	}
	if carry_bit > 0 {
	    result.insert(0, carry_bit);
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut digits: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    digits.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: Vec<i32> = Solution::plus_one(digits);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
