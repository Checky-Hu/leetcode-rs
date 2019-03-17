use std::env;

struct Solution {
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
	let mut prefix_c: char = '0';
        for c in s.chars() {
	    match c {
	        'I' => result += 1,
		'V' => {
		    match prefix_c {
		        'I' => result += 3,
			_ => result += 5,
		    }
		},
		'X' => {
		    match prefix_c {
		        'I' => result += 8,
		        _ => result += 10,
		    }
		},
		'L' => {
		    match prefix_c {
		        'X' => result += 30,
			_ => result += 50,
		    }
		},
		'C' => {
		    match prefix_c {
		        'X' => result += 80,
			_ => result += 100,
		    }
		},
		'D' => {
		    match prefix_c {
		        'C' => result += 300,
			_ => result += 500,
		    }
		},
		'M' => {
		    match prefix_c {
		        'C' => result += 800,
			_ => result += 1000,
		    }
		},
		_ => (),
	    }
	    prefix_c = c;
	}

	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let s: String = arg;
            println!("Integer: {}", Solution::roman_to_int(s));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
