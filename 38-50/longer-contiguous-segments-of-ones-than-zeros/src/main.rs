use std::cmp;
use std::env;

struct Solution {}

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut status: (char, i32, i32) = (' ', 0, 0);
        let mut counts: i32 = 0;
        for c in s.chars() {
            if status.0 == c {
                counts += 1;
            } else {
                if status.0 == '0' {
                    status.1 = cmp::max(status.1, counts);
                } else if status.0 == '1' {
                    status.2 = cmp::max(status.2, counts);
                }
                counts = 1;
                status.0 = c;
            }
        }
        if status.0 == '0' {
            status.1 = cmp::max(status.1, counts);
        } else if status.0 == '1' {
            status.2 = cmp::max(status.2, counts);
        }
        status.2 > status.1
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
                println!(
                    "Contiguous segment of ones longer: {}",
                    Solution::check_zero_ones(s)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
