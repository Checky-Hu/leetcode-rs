use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum: i32 = 0;
	let mut max: i32 = 0;
	let mut i: usize = 0;
	while i < nums.len() {
	    if i < k as usize {
	        sum += nums[i];
	    } else {
	        sum += nums[i] - nums[i - k as usize];
	    }
	    i += 1;
	    if i == k as usize || (i > k as usize && sum > max) {
	        max = sum;
	    }
	}
	f64::from(max) / f64::from(k)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    1 => k = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 >= k || k > ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Maximum average: {}", Solution::find_max_average(nums, k));
}

