use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	if n < k {
	    return result
	}

	let mut index: i32 = 0;
	while index < k {
	    if result.is_empty() {
	        for i in 1..=(n - k + 1) {
	            result.push(vec![i]);
		}
	    } else {
	        let mut tmp_result: Vec<Vec<i32>> = Vec::new();
		while !result.is_empty() {
		    tmp_result.push(result.pop().unwrap());
		}

	        for v in tmp_result {
		    let len: usize = v.len() - 1;
		    let mut tmp_i: i32 = v[len] + 1;
	            while tmp_i <= n - k + index + 1 {
		        let mut tmp_v: Vec<i32> = v.clone();
		        tmp_v.push(tmp_i);
		        result.push(tmp_v);
		        tmp_i += 1;
		    }
	        }
	    }
	    index += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut k: i32;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => n = i32::from_str(&arg).expect("Error parse."),
	    2 => {
	        ret = index;
                k = i32::from_str(&arg).expect("Error parse.");
                let result = Solution::combine(n, k);
                for v in result {
                    for n in v {
                        print!("{} ", n);
	            }
	            print!("\n");
                }
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }
}
