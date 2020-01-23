use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let t: i32 = target.abs();
        let mut result: i32 = 0;
        let mut sum: i32 = 0;
        while sum < t || (sum - t) & 1 == 1 {
            result += 1;
            sum += result;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let target: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Steps to reach {}: {}",
                target,
                Solution::reach_number(target)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
