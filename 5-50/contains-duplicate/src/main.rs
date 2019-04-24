use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len: usize = nums.len();
	if len == 0 || len == 1 {
	    return false
	}

	let mut map: HashMap<i32, bool> = HashMap::new();
	for i in 0..len {
	    if map.contains_key(&nums[i]) {
	        return true
	    } else {
	        map.insert(nums[i], true);
	    }
	}
	false
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

    println!("Contains: {}", Solution::contains_duplicate(nums));
}
