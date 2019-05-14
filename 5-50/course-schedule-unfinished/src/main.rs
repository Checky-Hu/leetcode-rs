use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if num_courses <= 0 {
	    return false
	}

	let mut count: i32 = 0;
        let mut record: Vec<bool> = vec![false; num_courses];
        for v in prerequisites {
	    if record[v[1] as usize] {
	        return false
	    } else {
	        record[v[1] as usize] = true;
	        count += 1;
		if count > num_courses {
		    return false
		}
	    }
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut prerequisites: Vec<Vec<i32>> = Vec::new();
    let mut len: i32 = 0;
    let mut tmp_v: Vec<i32> = Vec::new();
    let mut num_courses: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => num_courses = i32::from_str(&arg).expect("Error parse."),
	    2 => len = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
		tmp_v.push(number);
		if ret % 2 == 0 {
		    prerequisites.push(tmp_v);
		    tmp_v = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || len == 0 || len * 2 != ret as i32 {
        println!("Require at least (2 * len + 2) parameter.");
	return;
    }

    println!("Finish: {}", Solution::can_finish(num_courses, prerequisites));
}
