use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum: i32 = 0;
	for n in &nums {
	    sum += n;
	}
	if sum & 1 == 1 {
	    return false
	}

	sum /= 2;
	let mut result: Vec<bool> = vec![false; sum as usize + 1];
	result[0] = true;
	for n in &nums {
	    let mut i: usize = sum as usize;
	    while i >= *n as usize {
	        result[i] = result[i] || result[i - *n as usize];
		i -= 1;
	    }
	}
	result[sum as usize]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    nums.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Partition: {}", Solution::can_partition(nums));
}
