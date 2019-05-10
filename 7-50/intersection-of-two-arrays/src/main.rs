extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
	let len1: usize = nums1.len();
	let len2: usize = nums2.len();
        if len1 == 0 || len2 == 0 {
	    return result
	}

	let mut tmp_n1: Vec<i32> = nums1.clone();
	qsi32::quick_sort(&mut tmp_n1, 0, len1 - 1);
	let mut tmp_n2: Vec<i32> = nums2.clone();
	qsi32::quick_sort(&mut tmp_n2, 0, len2 - 1);
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut pre_n1: i32 = 0;
	while i < len1 {
	    if i == 0 || pre_n1 != tmp_n1[i] {
	        while j < len2 && tmp_n1[i] > tmp_n2[j] {
		    j += 1;
		}
		if j == len2 {
		    // Finish but not found, break.
		    break;
		} else {
		    pre_n1 = tmp_n1[i];
		    if tmp_n1[i] == tmp_n2[j] {
		        // Found intersection.
		        result.push(tmp_n1[i]);
			j += 1;
		    }
		}
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums1: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => {
	        let s1: String = arg;
		for s in s1.split_whitespace().collect::<Vec<_>>() {
		    nums1.push(i32::from_str(s).expect("Error parse."));
		}
	    },
	    2 => {
	        ret = index;
	        let s2: String = arg;
		let mut nums2: Vec<i32> = Vec::new();
		for s in s2.split_whitespace().collect::<Vec<_>>() {
		    nums2.push(i32::from_str(s).expect("Error parse."));
		}

                let result: Vec<i32> = Solution::intersection(nums1, nums2);
                print!("Intersection: ");
                for n in result {
                    print!("{} ", n);
                }
		print!("\n");
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
