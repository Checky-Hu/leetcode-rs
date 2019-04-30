use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
	if nums.is_empty() {
	    return result
	}

	let mut start: i32 = nums[0];
	let mut end: i32 = start;
	let mut i: usize = 1;
	while i < nums.len() {
	    if nums[i] == end + 1 {
	        end += 1;
	    } else {
	        let mut tmp_s: String = String::new();
		tmp_s.push_str(&start.to_string());
	        if end != start {
		    tmp_s.push_str("->");
		    tmp_s.push_str(&end.to_string());
		}
		result.push(tmp_s);
		start = nums[i];
		end = start;
	    }
	    i += 1;
	}

	let mut tmp_s: String = String::new();
	tmp_s.push_str(&start.to_string());
	if end != start {
	    tmp_s.push_str("->");
	    tmp_s.push_str(&end.to_string());
	}
	result.push(tmp_s);
	result
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

    let result: Vec<String> = Solution::summary_ranges(nums);
    for s in result {
        print!("{} ", s);
    }
    print!("\n");
}
