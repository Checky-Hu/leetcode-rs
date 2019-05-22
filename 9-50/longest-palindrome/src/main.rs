use std::env;

struct Solution {
}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut alphabets: Vec<i32> = vec![0; 52];
	for c in s.chars() {
	    let u: usize = c as usize;
	    if u >= 97 && u <= 122 {
	        alphabets[u - 97] += 1;
	    } else if u >= 65 && u <= 90 {
	        alphabets[u - 39] += 1;
	    } else {
	        continue;
	    }
	}
	let mut result: i32 = 0;
	let mut min: i32 = 0;
	for n in alphabets {
	    if n % 2 == 0 {
	        result += n;
	    } else {
	        if min == 0 {
		    min = n;
		    result += min;
		} else {
		    result += n - 1;
	            if n > min {
		        min = n;
		    }
		}
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
            let s: String = arg;
            println!("Length: {}", Solution::longest_palindrome(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameters.");
    }
}
