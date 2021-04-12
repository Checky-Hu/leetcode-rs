use std::env;

struct Solution {}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut is_one: bool = true;
        for c in s.chars() {
            if c == '1' {
                if !is_one {
                    return false;
                }
            } else {
                is_one = false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!("Check ones segment: {}", Solution::check_ones_segment(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
