use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n as usize);
        let target: i32 = n / 2;
        for i in 1..=target {
            result.push(0 - i);
            result.push(i);
        }
        if n & 1 == 1 {
            result.push(0);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<i32> = Solution::sum_zero(n);
            for r in &result {
                print!("{} ", *r);
            }
            println!();
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
