use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let min: usize = min_jump as usize;
        let max: usize = max_jump as usize;
        let len: usize = s.len();
        let mut status: Vec<bool> = vec![false; len];
        let mut prefix: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                status[0] = true;
            } else {
                if i >= min && status[i - min] {
                    prefix += 1;
                }
                if i > max && status[i - max - 1] {
                    prefix -= 1;
                }
                if prefix > 0 && c == '0' {
                    status[i] = true;
                }
            }
        }
        status[len - 1]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut min_jump: i32 = 0;
    let mut max_jump: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => min_jump = i32::from_str(&arg).expect("Error parse."),
            2 => max_jump = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let s: String = arg;
                println!("Can reach: {}", Solution::can_reach(s, min_jump, max_jump));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
