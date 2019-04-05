use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	let mut i: usize = 0;
	while i < num_rows as usize {
	    let mut j: usize = 1;
	    let mut tmp_v: Vec<i32> = vec![1];
	    while j < i {
	        tmp_v.push(result[i - 1][j - 1] + result[i - 1][j]);
		j += 1;
	    }
	    if i > 0 {
	        tmp_v.push(1);
	    }
	    result.push(tmp_v);
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let num_rows: i32 = i32::from_str(&arg).expect("Error parse.");
	    let result: Vec<Vec<i32>> = Solution::generate(num_rows);
	    for v in result {
	        for i in v {
                    print!("{} ", i);
		}
	        print!("\n");
	    }
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }
}
