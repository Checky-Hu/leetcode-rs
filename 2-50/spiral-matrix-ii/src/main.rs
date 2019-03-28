use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
	if n == 0 {
	    return result
	}

	for _i in 0..n {
	    result.push(vec![0; n as usize]);
	}
	let mut r_s: usize = 0;
	let mut r_e: usize = n as usize - 1;
	let mut c_s: usize = 0;
	let mut c_e: usize = n as usize - 1;
	let mut v: i32 = 1;
	loop {
	    if r_s == r_e && c_s == c_e {
	        result[r_s][c_s] = v;
		v += 1;
	    } else if r_s == r_e {
	        for x in c_s..=c_e {
		    result[r_s][x] = v;
		    v += 1;
		}
	    } else if c_s == c_e {
	        for y in r_s..=r_e {
		    result[y][c_s] = v;
		    v += 1;
		}
	    } else {
	        let mut x: usize = c_s;
	        while x < c_e {
	            result[r_s][x] = v;
		    v += 1;
		    x += 1;
	        }
	        let mut y: usize = r_s;
	        while y < r_e {
	            result[y][c_e] = v;
		    v += 1;
		    y += 1;
	        }
	        x = c_e;
	        while x > c_s {
	            result[r_e][x] = v;
		    v += 1;
		    if x > c_s + 1 {
		        x -= 1;
		    } else {
		        break;
		    }
	        }
	        y = r_e;
	        while y > r_s {
	            result[y][c_s] = v;
		    v += 1;
		    if y > r_s + 1 {
		        y -= 1;
		    } else {
		        break;
		    }
	        }
	    }
	    r_s += 1;
	    if r_e >= r_s + 1 {
	        r_e -= 1;
	    } else {
	        break;
	    }
	    c_s += 1;
	    if c_e >= c_s + 1 {
	        c_e -= 1;
	    } else {
	        break;
	    }
	}
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<Vec<i32>> = Solution::generate_matrix(n);
	    for v in result {
	        for n in v {
		    print!("{} ", n);
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
