use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
	let fizz: String = "Fizz".to_string();
	let buzz: String = "Buzz".to_string();
	let fizz_buzz: String = "FizzBuzz".to_string();
	for i in 1..=n {
	    if i % 3 == 0 {
	        if i % 5 == 0 {
		    result.push(fizz_buzz.clone());
		} else {
		    result.push(fizz.clone());
		}
	    } else if i % 5 == 0 {
	        result.push(buzz.clone());
	    } else {
	        result.push(i.to_string());
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    let result: Vec<String> = Solution::fizz_buzz(n);
            println!("Result:");
	    for s in result {
	        println!("{}", s);
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
