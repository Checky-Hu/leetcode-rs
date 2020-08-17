use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2 + ((low | high) & 1)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut low: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => low = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let high: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Odd numbers in interval range: {}",
                    Solution::count_odds(low, high)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
