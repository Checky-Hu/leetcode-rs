use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut tmp_vec1 = vec![0; 1024];
	for i in 1...9 {
	    tmp_str1.push();
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let n: i32  = i32::from_str(&arg).expect("Error parse.");
            println!("String: {}", Solution::count_and_say(n));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
