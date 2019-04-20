use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    fn find_peak_element_in_range(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
	if right - left <= 1 {
	    return -1
	}

	let mid: usize = left + (right - left) / 2;
	if nums[mid] > nums[mid - 1] && nums[mid] > nums[mid + 1] {
	    mid as i32
	} else {
	    let res1: i32 = Solution::find_peak_element_in_range(&nums, left, mid);
	    if res1 == -1 {
	        let res2: i32 = Solution::find_peak_element_in_range(&nums, mid, right);
	        if res2 == -1 {
		    -1
		} else {
		    res2
		}
	    } else {
	        res1
	    }
	}
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let result: i32 = Solution::find_peak_element_in_range(&nums, 0, len - 1);
	if result == -1 {
	    if len == 1 {
	        0
	    } else {
	        if nums[0] > nums[1] {
		    0
		} else {
		    len as i32 - 1
		}
	    }
	} else {
	    result
	}
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

    println!("Minimum: {}", Solution::find_peak_element(nums));
}
