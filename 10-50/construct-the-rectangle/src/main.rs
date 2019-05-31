use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; 2];
	let mut i: i32 = (area as f64).sqrt() as i32;
	while i > 0 {
	    if area % i == 0 {
	        result[0] = area / i;
		result[1] = i;
		break;
	    } else {
	        i -= 1;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let area: i32 = i32::from_str(&arg).expect("Error parse.");
	    let result: Vec<i32> = Solution::construct_rectangle(area);
            println!("Length: {}, Width: {}", result[0], result[1]);
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
