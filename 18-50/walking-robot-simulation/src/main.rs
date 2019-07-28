use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // 0 => up, 1 => right, 2 => down, 3 => left
        let mut direction: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
	let mut result: i32 = 0;
	for c in commands {
	    match c {
	        -2 => {
		    if direction == 0 {
		        direction = 3;
		    } else {
		        direction -= 1;
		    }
		},
		-1 => {
		    if direction == 3 {
		        direction = 0;
		    } else {
		        direction += 1;
		    }
		},
		1..=9 => {
		    if direction == 0 {
		        let mut tmp: i32 = y + c;
			for o in &obstacles {
			    if o[0] == x && o[1] > y && o[1] <= tmp {
			        tmp = o[1] - 1;
			    }
			}
			y = tmp;
		    } else if direction == 2 {
		        let mut tmp: i32 = y - c;
			for o in &obstacles {
			    if o[0] == x && o[1] < y && o[1] >= tmp {
			        tmp = o[1] + 1;
			    }
			}
			y = tmp;
		    } else if direction == 1 {
		        let mut tmp: i32 = x + c;
			for o in &obstacles {
			    if o[1] == y && o[0] > x && o[0] <= tmp {
			        tmp = o[0] - 1;
			    }
			}
			x = tmp;
		    } else {
		        let mut tmp: i32 = x - c;
			for o in &obstacles {
			    if o[1] == y && o[0] < x && o[0] >= tmp {
			        tmp = o[0] + 1;
			    }
			}
			x = tmp;
		    }
	            let cur: i32 = x * x + y * y;
		    if result < cur {
		        result = cur;
		    }
		},
		_ => (),
	    }
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut count: i32 = 0;
    let mut commands: Vec<i32> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    let mut obstacles: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => count = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
		if ret <= count {
		    commands.push(number);
		} else {
		    tmp.push(number);
		    if tmp.len() == 2 {
		        obstacles.push(tmp);
		        tmp = Vec::new();
		    }
		}
	    },
	}
    }

    if 0 == ret || count > ret {
        println!("Require at least (count + 2 * n) parameters.");
	return;
    }

    println!("Distance: {}", Solution::robot_sim(commands, obstacles));
}
