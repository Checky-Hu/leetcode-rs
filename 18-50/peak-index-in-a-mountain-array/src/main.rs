use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut result: usize = 1;
	while result < a.len() - 1 {
	    if a[result] > a[result + 1] {
	        break;
	    } else {
	        result += 1;
	    }
	}
	result as i32
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

    if 3 > ret {
        println!("Require at least three parameters.");
	return;
    }

    println!("Index: {}", Solution::peak_index_in_mountain_array(a));
}

