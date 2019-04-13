use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; 2];
	if numbers.is_empty() {
	    return result
	}

	let mut left: usize = 0;
	let mut right: usize = numbers.len() - 1;
	while left < right {
	    let sum: i32 = numbers[left] + numbers[right];
	    if sum == target {
	        result[0] = left as i32 + 1;
	        result[1] = right as i32 + 1;
		break;
	    } else if sum > target {
	        right -= 1;
	    } else {
	        left += 1;
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut numbers: Vec<i32> = Vec::new();
    let mut target: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => target = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        numbers.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    let result: Vec<i32> = Solution::two_sum(numbers, target);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
