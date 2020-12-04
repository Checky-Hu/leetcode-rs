use std::env;

struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut counts: i32 = 0;
        let mut result: i32 = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    counts += 1;
                    if counts > result {
                        result = counts;
                    }
                }
                ')' => {
                    counts -= 1;
                }
                _ => (),
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                println!("Maximum nesting depth: {}", Solution::max_depth(arg));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
