use std::env;

struct Solution {
}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut flags: Vec<i32> = vec![0; 128];
	for c in s.chars() {
	    flags[(c as u8) as usize] += 1;
	}

	let mut result: String = String::new();
	loop {
	    let mut max: i32 = 0;
	    let mut index: i32 = -1;
	    for i in 0..128 {
	        if flags[i] > max {
		    max = flags[i];
		    index = i as i32;
		}
	    }
	    if index >= 0 {
	        for _i in 0..max {
		    result.push((index as u8) as char);
		}
		flags[index as usize] = 0;
	    } else {
	        break;
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
            println!("String: {}", Solution::frequency_sort(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
