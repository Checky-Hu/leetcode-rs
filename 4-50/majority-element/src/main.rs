use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut flag: i32 = 0;
	let mut count: i32 = 0;
        for n in nums {
	    if count == 0 {
	        flag = n;
		count = 1;
	    } else {
	        if flag == n {
	            count += 1;
	        } else {
		    count -= 1;
		    if count == 0 {
		        flag = n;
			count = 1;
		    }
		}
	    }
	}
	flag
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

    println!("Element: {}", Solution::majority_element(nums));
}
