use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut result: i32 = num_bottles;
        let mut counts: i32 = num_bottles;
        while counts >= num_exchange {
            let t: i32 = counts / num_exchange;
            result += t;
            counts = counts % num_exchange + t;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut num_bottles: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => num_bottles = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num_exchange: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Number of drinks: {}",
                    Solution::num_water_bottles(num_bottles, num_exchange)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
