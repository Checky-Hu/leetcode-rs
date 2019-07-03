use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut points: Vec<i32> = Vec::new();
	for s in ops {
	    if s == "+" {
	        let len: usize = points.len();
	        points.push(points[len - 2] + points[len - 1]);
	    } else if s == "D" {
	        let len: usize = points.len();
	        points.push(points[len - 1] * 2);
	    } else if s == "C" {
	        points.pop();
	    } else {
	        points.push(i32::from_str(&s).expect("Error parse."));
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

