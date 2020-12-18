use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Number of matches: {}", Solution::number_of_matches(n));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
