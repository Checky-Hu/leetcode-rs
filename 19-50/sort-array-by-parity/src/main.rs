use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let len: usize = a.len();
	let mut result: Vec<i32> = vec![0; len];
	let mut even_pos: usize = 0;
	let mut odd_pos: usize = len - 1;
	for n in a {
	    if n % 2 == 0 {
	        result[even_pos] = n;
		even_pos += 1;
	    } else {
	        result[odd_pos] = n;
		if odd_pos > 0 {
		    odd_pos -= 1;
		} else {
		    break;
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    a.push(n);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result: Vec<i32> = Solution::sort_array_by_parity(a);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}

