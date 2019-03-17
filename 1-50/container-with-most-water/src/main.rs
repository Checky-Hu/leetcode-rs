use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
	let mut index: usize = 0;
	let len: usize = height.len();
	while index < len - 1 {
	    let mut tmp_area: i32 = 0;
	    let mut tmp_i: usize = 1;
	    while index + tmp_i < len {
	        let area: i32 = match height[index] > height[index + tmp_i] {
		    true => height[index + tmp_i] * tmp_i as i32,
		    false => height[index] * tmp_i as i32,
		};

		if area > tmp_area {
		    tmp_area = area;
		}
		tmp_i += 1;
	    }

	    if tmp_area > result {
	        result = tmp_area;
	    }
	    index += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut height: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    height.push(number);
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    let area: i32 = Solution::max_area(height);
    println!("Area: {}", area);
}
