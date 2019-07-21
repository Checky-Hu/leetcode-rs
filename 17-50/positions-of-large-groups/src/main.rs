use std::env;

struct Solution {
}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	let mut start: usize = 0;
	let mut end: usize = 0;
	let mut i: usize = 0;
	let mut cur_c: char = '\0';
	for c in s.chars() {
	    if cur_c == c {
	        end = i;
	    } else {
		if end - start > 1 {
		    result.push(vec![start as i32, end as i32]);
		}
		start = i;
		end = i;
		cur_c = c;
	    }
	    i += 1;
	}
	if end - start > 1 {
	    result.push(vec![start as i32, end as i32]);
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
            ret += 1;
            let s: String = arg;
	    let result: Vec<Vec<i32>> = Solution::large_group_positions(s);
	    for v in result {
                println!("s: {}, e: {}", v[0], v[1]);
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

