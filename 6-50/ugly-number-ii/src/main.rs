use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut nums: Vec<i32> = vec![1];
	let mut index: usize = 1;
	let mut i2: usize = 0;
	let mut i3: usize = 0;
	let mut i5: usize = 0;
	while index < n as usize {
	    let p2: i32 = nums[i2] * 2;
	    let p3: i32 = nums[i3] * 3;
	    let p5: i32 = nums[i5] * 5;
	    let min: i32 = if p2 <= p3 {
	        if p2 <= p5 {
		    p2
		} else {
		    p5
		}
	    } else {
	        if p5 <= p3 {
		    p5
		} else {
		    p3
		}
	    };
	    if min == p2 {
	        i2 += 1;
	    }
	    if min == p3 {
	        i3 += 1;
	    }
	    if min == p5 {
	        i5 += 1;
	    }
	    nums.push(min);
	    index += 1;
	}
	nums[n as usize - 1]
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Number: {}", Solution::nth_ugly_number(n));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
