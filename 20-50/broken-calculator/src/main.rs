use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut result: i32 = 0;
        let mut y_mut: i32 = y;
        while y_mut > x {
            result += 1;
            if y_mut & 1 == 1 {
                y_mut += 1;
            } else {
                y_mut /= 2;
            }
        }
        result + x - y_mut
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut x: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            x = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            ret += 1;
            let y: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Minimum operations from {} to {}: {}",
                x,
                y,
                Solution::broken_calc(x, y)
            );
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
