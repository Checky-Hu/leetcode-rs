use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut result: Vec<Vec<i32>> = vec![vec![0; 3]; 50000];
	let mut i: i32 = 0;
	for n in nums {
	    result[n as usize][0] += 1;
	    if result[n as usize][0] == 1 {
	        result[n as usize][1] = i - 1;
	    }
	    result[n as usize][2] = i;
	    i += 1;
	}
	let mut max_count: i32 = 0;
	let mut min_len: i32 = 0;
	for v in result {
	    if v[0] > max_count {
	        max_count = v[0];
	        min_len = v[2] - v[1];
	    } else if v[0] == max_count {
	        if min_len > v[2] - v[1] {
		    min_len = v[2] - v[1];
		}
	    }
	}
	min_len
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Length: {}", Solution::find_shortest_sub_array(nums));
}

