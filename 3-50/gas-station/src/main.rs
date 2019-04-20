use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
	let mut result: usize = 0;
	let mut total: i32 = 0;
	let mut remain: i32 = 0;
	for i in 0..gas.len() {
	    total += gas[i] - cost[i];
	    remain += gas[i] - cost[i];
	    if remain < 0 {
	        result = i + 1;
		remain = 0;
	    }
	}
	if total < 0 {
	    -1
	} else {
	    result as i32
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut len: i32 = 0;
    let mut gas: Vec<i32> = Vec::new();
    let mut cost: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => len = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        if ret > len {
		    cost.push(number);
		} else {
		    gas.push(number);
		}
	    },
	}
    }

    if len == 0 || len * 2 != ret {
        println!("Require at least (len * 2 + 1) parameter.");
	return;
    }

    println!("Index: {}", Solution::can_complete_circuit(gas, cost));
}
