use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
	let mut result: i32 = 0;
	let mut index: usize = 1;
	while index < prices.len() {
	    if prices[index] > prices[index - 1] {
	        result += prices[index] - prices[index - 1];
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
