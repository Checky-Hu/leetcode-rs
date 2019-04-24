use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
	let mut map: HashMap<i32, usize> = HashMap::new();
	let mut i: usize = 0;
	for n in nums {
	    if map.contains_key(&n) && i - map.get(&n).unwrap() <= k as usize {
	        return true
	    } else {
	        map.insert(n, i);
		i += 1;
	    }
	}
	false
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => k = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    println!("Contains: {}", Solution::contains_nearby_duplicate(nums, k));
}
