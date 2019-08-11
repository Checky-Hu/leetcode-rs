use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
	let mut result: Vec<i32> = Vec::new();
	let mut x_power: Vec<i32> = Vec::new();
	if x == 1 {
	    x_power.push(1);
	} else {
	    let mut tmp_x: i32 = 1;
	    while tmp_x <= bound {
	        x_power.push(tmp_x);
		tmp_x *= x;
	    }
	}
	let mut y_power: Vec<i32> = Vec::new();
	if y == 1 {
	    y_power.push(1);
	} else {
	    let mut tmp_y: i32 = 1;
	    while tmp_y <= bound {
	        y_power.push(tmp_y);
		tmp_y *= y;
	    }
	}
	let mut set: HashSet<i32> = HashSet::new();
	for v1 in x_power {
	    for v2 in &y_power {
	        let tmp: i32 = v1 + v2;
		if tmp <= bound && !set.contains(&tmp) {
		    set.insert(tmp);
		    result.push(tmp);
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut bound: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => x = i32::from_str(&arg).expect("Error parse."),
            2 => y = i32::from_str(&arg).expect("Error parse."),
            3 => {
	        ret = 3;
	        bound = i32::from_str(&arg).expect("Error parse.");
		break;
	    },
	    _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    let result: Vec<i32> = Solution::powerful_integers(x, y, bound);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}

