use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut n1: i32 = p;
        let mut n2: i32 = q;
        while n1 & 1 == 0 && n2 & 1 == 0 {
            n1 >>= 1;
            n2 >>= 1;
        }
        if n1 & 1 == 0 {
            2
        } else if n2 & 1 == 0 {
            0
        } else {
            1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut p: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => p = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let q: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Final receptor: {}", Solution::mirror_reflection(p, q));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
