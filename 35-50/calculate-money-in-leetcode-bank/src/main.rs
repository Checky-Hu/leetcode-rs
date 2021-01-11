use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let week: i32 = n / 7;
        let day: i32 = n % 7;
        (49 + week * 7) * week / 2 + (week * 2 + 1 + day) * day / 2
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
                println!("Total money: {}", Solution::total_money(n));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
