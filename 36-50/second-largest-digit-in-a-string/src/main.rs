use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut result: (i32, i32) = (-1, -1);
        for c in s.chars() {
            if c.is_numeric() {
                let t: i32 = c as i32 - 48;
                match t.cmp(&result.0) {
                    Ordering::Less => {
                        if t > result.1 {
                            result.1 = t;
                        }
                    }
                    Ordering::Equal => (),
                    Ordering::Greater => {
                        result.1 = result.0;
                        result.0 = t;
                    }
                }
            }
        }
        result.1
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
                println!("Second largest digit: {}", Solution::second_highest(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
