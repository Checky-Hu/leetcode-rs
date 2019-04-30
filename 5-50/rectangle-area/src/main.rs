use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let (r0_xs, r0_xe) = if a <= c {
	    (a, c)
	} else {
	    (c, a)
	};
        let (r0_ys, r0_ye) = if b <= d {
	    (b, d)
	} else {
	    (d, b)
	};
        let (r1_xs, r1_xe) = if e <= g {
	    (e, g)
	} else {
	    (g, e)
	};
        let (r1_ys, r1_ye) = if f <= h {
	    (f, h)
	} else {
	    (h, f)
	};

	let result: i32 = (r0_xe - r0_xs) * (r0_ye - r0_ys) + (r1_xe - r1_xs) * (r1_ye - r1_ys);
	// println!("result: {}", result);
	let cover_x: i32;
	if r0_xs <= r1_xs {
	    if r1_xs < r0_xe {
	        if r1_xe <= r0_xe {
		    cover_x = r1_xe - r1_xs;
		} else {
		    cover_x = r0_xe - r1_xs;
		}
	    } else {
	        return result
	    }
	} else {
	    if r0_xs < r1_xe {
	        if r0_xe <= r1_xe {
		    cover_x = r0_xe - r0_xs;
		} else {
		    cover_x = r1_xe - r0_xs;
		}
	    } else {
	        return result
	    }
	}
	let cover_y: i32;
	if r0_ys <= r1_ys {
	    if r1_ys < r0_ye {
	        if r1_ye <= r0_ye {
		    cover_y = r1_ye - r1_ys;
		} else {
		    cover_y = r0_ye - r1_ys;
		}
	    } else {
	        return result
	    }
	} else {
	    if r0_ys < r1_ye {
	        if r0_ye <= r1_ye {
		    cover_y = r0_ye - r0_ys;
		} else {
		    cover_y = r1_ye - r0_ys;
		}
	    } else {
	        return result
	    }
	}
	// println!("x: {} y: {}", cover_x, cover_y);
	result - cover_x * cover_y
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut vector: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
	    vector.push(number);
	}
    }

    if 8 != ret {
        println!("Require at least 8 parameters.");
	return;
    }

    println!("Area: {}", Solution::compute_area(vector[0], vector[1], vector[2], vector[3],
	vector[4], vector[5], vector[6], vector[7]));
}
