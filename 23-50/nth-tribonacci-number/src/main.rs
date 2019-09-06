use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let mut t0: i32 = 0;
                let mut t1: i32 = 1;
                let mut t2: i32 = 1;
                let mut i: i32 = 3;
                while i <= n {
                    let tmp: i32 = t0 + t1 + t2;
                    t0 = t1;
                    t1 = t2;
                    t2 = tmp;
                    i += 1;
                }
                t2
            }
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Tribonacci: {}", Solution::tribonacci(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
