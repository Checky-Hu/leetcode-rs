use std::env;

struct Solution {}

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        // "0101"
        let mut zero1: i32 = 0;
        let mut one1: i32 = 0;
        // "1010"
        let mut zero2: i32 = 0;
        let mut one2: i32 = 0;
        let mut current: char = '0';
        for c in s.chars() {
            if current == '0' {
                if c == '0' {
                    one2 += 1;
                } else {
                    zero1 += 1;
                }
                current = '1';
            } else {
                if c == '0' {
                    one1 += 1;
                } else {
                    zero2 += 1;
                }
                current = '0';
            }
        }
        if zero1 == one1 {
            if zero2 == one2 {
                if zero1 < zero2 {
                    zero1
                } else {
                    zero2
                }
            } else {
                zero1
            }
        } else if zero2 == one2 {
            zero2
        } else {
            -1
        }
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
                println!("Minimum swaps: {}", Solution::min_swaps(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
