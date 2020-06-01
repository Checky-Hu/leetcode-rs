use std::env;

struct Solution {}

impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut t1: Vec<u8> = s1.into_bytes();
        t1.sort();
        let mut t2: Vec<u8> = s2.into_bytes();
        t2.sort();
        let mut break1: bool = true;
        let mut break2: bool = true;
        for (i, v) in t1.iter().enumerate() {
            if *v > t2[i] {
                break2 = false;
            } else if *v < t2[i] {
                break1 = false;
            }
        }
        break1 | break2
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s1 = arg,
            _ => {
                ret += 1;
                let s2: String = arg;
                println!(
                    "Check if can break: {}",
                    Solution::check_if_can_break(s1, s2)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
