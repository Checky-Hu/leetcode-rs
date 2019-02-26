use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { false }
	else if x % 10 == 0 && x != 0 { false }
	else {
	    let mut half_number: i32 = 0;
	    let mut tmp_x: i32 = x;
	    while tmp_x > half_number {
	        half_number = half_number * 10 + tmp_x % 10;
		tmp_x /= 10;
	    }
	    tmp_x == half_number || tmp_x == half_number / 10
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");

            match Solution::is_palindrome(number) {
                true => println!("It's palindrome number."),
                false => println!("It isn't palindrome number."),
	    }
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
