use std::env;

struct Solution {
}

impl Solution {
    pub fn original_digits(s: String) -> String {
        // zero, one, two, three, four, five, six, seven, eight, nine
        // 'z' -> 0, 'w' -> 2, 'u' -> 4, 'x' -> 6, 'g' -> 8
        // 'o' - 'z' - 'w' - 'u' -> 1, 'h' - 'g' -> 3, 'f' - 'u' -> 5, 's' - 'x' -> 7, 'i' - '5' - '6' - '8' -> 9
        let mut count_z: i32 = 0;
	let mut count_w: i32 = 0;
	let mut count_u: i32 = 0;
	let mut count_x: i32 = 0;
	let mut count_g: i32 = 0;
	let mut count_o: i32 = 0;
	let mut count_h: i32 = 0;
	let mut count_f: i32 = 0;
	let mut count_s: i32 = 0;
	let mut count_i: i32 = 0;
	for c in s.chars() {
	    match c {
	        'z' => count_z += 1,
	        'w' => count_w += 1,
	        'u' => count_u += 1,
	        'x' => count_x += 1,
	        'g' => count_g += 1,
	        'o' => count_o += 1,
	        'h' => count_h += 1,
	        'f' => count_f += 1,
	        's' => count_s += 1,
	        'i' => count_i += 1,
		_ => (),
	    }
	}
	let mut count: Vec<i32> = vec![0; 10];
	count[0] = count_z;
	count[1] = count_o - count_z - count_w - count_u;
	count[2] = count_w;
	count[3] = count_h - count_g;
	count[4] = count_u;
	count[5] = count_f - count_u;
	count[6] = count_x;
	count[7] = count_s - count_x;
	count[8] = count_g;
	count[9] = count_i - count[5] - count[6] - count[8];
	let mut result: String = String::new();
	let mut i: usize = 0;
	while i < 10 {
	    let mut j: i32 = 0;
	    while j < count[i] {
	        result.push((i as u8 + 48) as char);
		j += 1;
	    }
	    i += 1;
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Digits: {}", Solution::original_digits(s));
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameters.");
    }
}
