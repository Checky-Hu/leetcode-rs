use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
	let len: usize = nums2.len();
	for n in nums1 {
	     let mut i: usize = 0;
	     while i < len {
	         if nums2[i] == n {
		     break;
		 }
		 i += 1;
	     }
	     i += 1;
	     while i < len {
	         if nums2[i] > n {
		     break;
		 }
		 i += 1;
	     }
	     if i < len {
	         result.push(nums2[i]);
	     } else {
	         result.push(-1);
	     }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            n = i32::from_str(&arg).expect("Error parse.");
	} else if 1 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    if nums1.len() < n as usize {
	        nums1.push(number);
	    } else {
	        nums2.push(number);
	    }
	}
    }

    if 0 == ret || n as usize > ret {
        println!("Require at least (n + 1) parameters.");
	return;
    }

    let result: Vec<i32> = Solution::next_greater_element(nums1, nums2);
    for r in result {
        print!("{} ", r);
    }
    print!("\n");
}
