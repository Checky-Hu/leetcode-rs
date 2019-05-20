use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max: Vec<i32> = vec![i32::min_value(); 3];
	let mut count: i32 = 0;
	for n in nums {
	    if n == max[0] {
	        if count == 0 {
	            count += 1;
		}
	    } else if n > max[0] {
	        max[2] = max[1];
		max[1] = max[0];
		max[0] = n;
		count += 1;
	    } else {
	        if n == max[1] {
		    if count == 1 {
		        count += 1;
		    }
		} else if n > max[1] {
		    max[2] = max[1];
		    max[1] = n;
		    count += 1;
		} else {
		    if n == max[2] {
		        if count == 2 {
			    count += 1;
			}
		    } else if n > max[2] {
		        max[2] = n;
			count += 1;
		    } else {
		        count += 1;
		    }
		}
	    }
	}
	if count < 3 {
	    max[0]
	} else {
	    max[2]
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

    println!("Third max: {}", Solution::third_max(nums));
}
