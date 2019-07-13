use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut n_2: i32 = cost[0];
        let mut n_1: i32 = cost[1];
	let mut i: usize = 2;
	while i < cost.len() {
	    let tmp: i32 = cost[i] + if n_2 < n_1 {
	        n_2
	    } else {
	        n_1
	    };
	    n_2 = n_1;
	    n_1 = tmp;
	    i += 1;
	}
	if n_2 < n_1 {
	    n_2
	} else {
	    n_1
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut cost: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        cost.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Min cost: {}", Solution::min_cost_climbing_stairs(cost));
}

