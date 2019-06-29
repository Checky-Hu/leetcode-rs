use std::env;

struct Solution {
}

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut result: Vec<i32> = vec![0; 2];
	for c in moves.chars() {
	    match c {
	        'R' => result[0] += 1,
	        'L' => result[0] -= 1,
	        'U' => result[1] += 1,
	        'D' => result[1] -= 1,
		_ => (),
	    }
	}
	result[0] == 0 && result[1] == 0
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
	if 1 == index {
	    ret = 1;
	    let moves: String = arg;
            println!("Return origin: {}", Solution::judge_circle(moves));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
