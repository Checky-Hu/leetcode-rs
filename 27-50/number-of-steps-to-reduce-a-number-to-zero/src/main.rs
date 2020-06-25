use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut mut_num: i32 = num;
        let mut result: i32 = 0;
        while mut_num > 0 {
            if mut_num & 1 == 1 {
                mut_num -= 1;
            } else {
                mut_num >>= 1;
            }
            result += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Number of steps to reduce zero: {}",
                    Solution::number_of_steps(num)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
