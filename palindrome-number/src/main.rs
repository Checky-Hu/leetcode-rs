use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { false }
	else if x < 10 { true }
	else { true }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");

            match Solution::is_palindrome(number) {
                true => println!("true"),
                false => println!("false"),
	    }
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => println!("Finish."),
    }
}
