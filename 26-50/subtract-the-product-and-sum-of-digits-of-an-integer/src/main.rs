use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut tmp: i32 = n;
        let mut mul: i32 = 1;
        let mut sum: i32 = 0;
        loop {
            let cur: i32 = tmp % 10;
            mul *= cur;
            sum += cur;
            tmp /= 10;
            if tmp == 0 {
                break;
            }
        }
        mul - sum
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Subtract: {}", Solution::subtract_product_and_sum(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
