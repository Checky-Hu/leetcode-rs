extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn combination_sum_loop(candidates: &Vec<i32>, first: usize, target: i32) -> Vec<Vec<i32>> {
        if candidates.len() <= first || candidates[first] > target {
	    return vec![]
	}

	let mut result: Vec<Vec<i32>> = Vec::new();
	let mut index: usize = first;
	let mut preceding_c: i32 = candidates[first];
	while index < candidates.len() {
	    if preceding_c == candidates[index] && index != first {
	        index += 1;
		continue;
	    }

	    if candidates[index] == target {
	        let tmp_result: Vec<i32> = vec![target];
		result.push(tmp_result);
		preceding_c = candidates[index];
		index += 1;
	    } else if candidates[index] < target {
		let rest_result: Vec<Vec<i32>> = Solution::combination_sum_loop(candidates, index + 1, target - candidates[index]);
		if !rest_result.is_empty() {
		    for mut v in rest_result {
	                v.insert(0, candidates[index]);
			result.push(v);
		    }
		}
		preceding_c = candidates[index];
	        index += 1;
	    } else {
	        break;
	    }
	}
	result
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut tmp_data: Vec<i32> = candidates.clone();
	qsi32::quick_sort(&mut tmp_data, 0, candidates.len() - 1);
	Solution::combination_sum_loop(&tmp_data, 0, target)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut candidates: Vec<i32> = Vec::new();
    let mut target: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => target = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        candidates.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    let result: Vec<Vec<i32>> = Solution::combination_sum2(candidates, target);
    for v in result {
        for n in v {
            print!("{} ", n);
	}
	print!("\n");
    }
}
