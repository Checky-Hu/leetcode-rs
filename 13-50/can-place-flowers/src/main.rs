use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut result: i32 = 0;
        let mut count: i32 = 0;
	let mut i: usize = 0;
	while i < flowerbed.len() {
	    if flowerbed[i] == 0 {
	        count += 1;
		i += 1;
	    } else {
	        result += count / 2;
		count = 0;
	        i += 2;
	    }
	}
	result += (count + 1) / 2;
	if result >= n {
	    true
	} else {
	    false
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut flowerbed: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
	    1 => n = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        flowerbed.push(number);
	    },
	}
    }

    if 2 > ret {
        println!("Require at least two parameters.");
	return;
    }

    println!("Can place: {}", Solution::can_place_flowers(flowerbed, n));
}
