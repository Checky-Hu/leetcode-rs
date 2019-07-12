use std::env;

struct Solution {
}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
	for c in &letters {
	    if *c > target {
	        return *c
	    }
	}
	letters[0]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: char = '\0';
    let mut letters: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	match index {
	    0 => (),
            1 => target = arg.chars().next().unwrap(),
	    _ => {
                ret += 1;
                let c: char = arg.chars().next().unwrap();
	        letters.push(c);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Next greatest letter: {}", Solution::next_greatest_letter(letters, target));
}

