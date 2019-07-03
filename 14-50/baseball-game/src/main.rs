use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut points: Vec<i32> = Vec::new();
	let mut len: usize = 0;
	for s in ops {
	    if s == "+" {
	        points.push(points[len - 2] + points[len - 1]);
		len += 1;
	    } else if s == "D" {
	        points.push(points[len - 1] * 2);
		len += 1;
	    } else if s == "C" {
	        points.pop();
		len -= 1;
	    } else {
	        points.push(i32::from_str(&s).expect("Error parse."));
		len += 1;
	    }
	}
	let mut result: i32 = 0;
	for n in points {
	    result += n;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut ops: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    _ => {
                ret += 1;
                let op: String = arg;
	        ops.push(op);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Points: {}", Solution::cal_points(ops));
}

