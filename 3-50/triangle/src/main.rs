use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
	    return 0
	}

	let len: usize = triangle.len();
	let mut tmp_v: Vec<i32> = vec![0; len];
	for i in 0..len {
	    if i > 0 {
	        tmp_v[i] = tmp_v[i - 1] + triangle[i][i];
	        let mut tmp_i: usize = i - 1;
	        while tmp_i > 0 {
	            tmp_v[tmp_i] = triangle[i][tmp_i] + if tmp_v[tmp_i] > tmp_v[tmp_i - 1] {
		        tmp_v[tmp_i - 1]
		    } else {
		        tmp_v[tmp_i]
		    };
	 	    tmp_i -= 1;
	        }
	    }
	    tmp_v[0] += triangle[i][0];
	}

	let mut result: i32 = tmp_v[0];
	for i in tmp_v {
	    if i < result {
	        result = i;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut triangle: Vec<Vec<i32>> = Vec::new();
    let mut tmp_v: Vec<i32> = Vec::new();
    let mut count: i32 = 1;
    let mut tmp_count: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    tmp_v.push(number);
	    tmp_count += 1;

	    if tmp_count == count {
	        triangle.push(tmp_v);
	        tmp_v = Vec::new();
		tmp_count = 0;
		count += 1;
	    }
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Minimum total: {}", Solution::minimum_total(triangle));
}
