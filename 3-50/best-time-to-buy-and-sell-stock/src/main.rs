use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_p: i32 = 0;
	let mut result: i32 = 0;
	let mut index: usize = 0;
	while index < prices.len() {
	    if index == 0 {
	        min_p = prices[index];
	    } else {
	        if prices[index] < min_p {
	            min_p = prices[index];
		}
		let tmp: i32 = prices[index] - min_p;
		if tmp > result {
		    result = tmp;
		}
	    }
	    index += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut prices: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    prices.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    ;
    println!("Max profit: {}", Solution::max_profit(prices));
}
