use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut result: Vec<i32> = Vec::new();
	for s in tokens {
	    match s.as_str() {
	        "+" => {
		    let n1: i32 = result.pop().unwrap();
		    let n2: i32 = result.pop().unwrap();
		    result.push(n2 + n1);
		},
	        "-" => {
		    let n1: i32 = result.pop().unwrap();
		    let n2: i32 = result.pop().unwrap();
		    result.push(n2 - n1);
		},
	        "*" => {
		    let n1: i32 = result.pop().unwrap();
		    let n2: i32 = result.pop().unwrap();
		    result.push(n2 * n1);
		},
	        "/" => {
		    let n1: i32 = result.pop().unwrap();
		    let n2: i32 = result.pop().unwrap();
		    result.push(n2 / n1);
		},
		_ => {
		    let n: i32 = i32::from_str(&s).expect("Parse error.");
		    result.push(n);
		},
	    }
	}
	result[0]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut tokens: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
	    tokens.push(arg);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Value: {}", Solution::eval_rpn(tokens));
}
