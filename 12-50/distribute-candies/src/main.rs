use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let target: usize = candies.len() / 2;
	let mut flags: Vec<i32> = vec![0; 200001];
	for n in candies {
	    flags[(n + 100000) as usize] += 1;
	}
	let mut result: i32 = 0;
	for i in 0..200001 {
	    if flags[i] > 0 {
	        result += 1;
		if result == target as i32 {
		    break;
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut candies: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    candies.push(number);
	}
    }

    if 2 > ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Kinds: {}", Solution::distribute_candies(candies));
}
