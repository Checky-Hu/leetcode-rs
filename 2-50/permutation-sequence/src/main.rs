use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut nums: Vec<u8> = vec![49, 50, 51, 52, 53, 54, 55, 56, 57];
	let mut v: Vec<usize> = vec![1; n as usize];
	let mut i: usize = 1;
	while i < n as usize {
	    v[i] = v[i - 1] * i;
	    i += 1;
	}

	let mut result: String = String::new();
	let mut pos: usize = k as usize - 1;
	i = n as usize;
	while i >= 1 {
	    let tmp: usize = pos / v[i - 1];
	    pos %= v[i - 1];
	    result.push(nums[tmp] as char);
	    nums.remove(tmp);
	    i -= 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => n = i32::from_str(&arg).expect("Error parse."),
	    2 => {
                ret = index;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("String: {}", Solution::get_permutation(n, k));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
