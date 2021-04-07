use std::env;

struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count_0: i32 = 0;
        let mut count_1: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if i & 1 == 0 {
                if c == '0' {
                    count_1 += 1;
                } else {
                    count_0 += 1;
                }
            } else if c == '0' {
                count_0 += 1;
            } else {
                count_1 += 1;
            }
        }
        if count_0 < count_1 {
            count_0
        } else {
            count_1
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
                println!("Minimum operations: {}", Solution::min_operations(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
