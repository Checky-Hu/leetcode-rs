use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        // [1, 2, 4, 8]
        let hours: Vec<Vec<u8>> = vec![vec![0], vec![1, 2, 4, 8], vec![3, 5, 9, 6, 10], vec![7, 11]];
	// [1, 2, 4, 8, 16, 32]
	let minutes: Vec<Vec<u8>> = vec![vec![0], vec![1, 2, 4, 8, 16, 32],
	    vec![3, 5, 9, 17, 33, 6, 10, 18, 34, 12, 20, 36, 24, 40, 48],
	    vec![7, 11, 19, 35, 13, 21, 37, 25, 41, 49, 14, 22, 38, 26, 42, 50, 28, 44, 52, 56],
	    vec![15, 23, 39, 27, 43, 51, 29, 45, 53, 57, 30, 46, 54, 58],
	    vec![31, 47, 59, 55]];
	let mut result: Vec<String> = Vec::new();
	let mut h: i32 = 0;
	while h <= num {
	    if h > 3 {
	        break;
	    } else {
	        let m: i32 = num - h;
		if m <= 5 {
		    for hour in &hours[h as usize] {
		        let mut s: String = String::new();
		        if hour / 10 > 0 {
			    s.push((hour / 10 + 48) as char);
			}
			s.push((hour % 10 + 48) as char);
			s.push(':');
		        for minute in &minutes[m as usize] {
			    let mut tmp: String = s.clone();
			    tmp.push((minute / 10 + 48) as char);
			    tmp.push((minute % 10 + 48) as char);
			    result.push(tmp);
			}
		    }
		}
		h += 1;
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
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
	    let result: Vec<String> = Solution::read_binary_watch(num);
            println!("Result:");
	    for s in result {
	        println!("{}", s);
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
