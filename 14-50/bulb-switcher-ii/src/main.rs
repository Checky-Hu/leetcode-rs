use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn flip_lights(n: i32, m: i32) -> i32 {
        if n == 0 || m == 0 {
            return 1;
        }
        if n == 1 {
            2
        } else if n == 2 {
            match m {
                1 => 3,
                _ => 4,
            }
        } else {
            match m {
                1 => 4,
                2 => 7,
                _ => 8,
            }
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let m: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Result count: {}", Solution::flip_lights(n, m));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
