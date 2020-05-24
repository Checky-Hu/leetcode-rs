use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut current_a: i32 = 0;
        let mut current_b: i32 = 0;
        let mut current_c: i32 = 0;
        let mut remain_a: i32 = a;
        let mut remain_b: i32 = b;
        let mut remain_c: i32 = c;
        let mut result: String = String::new();
        loop {
            if remain_a > 0
                && ((remain_a >= remain_b && remain_a >= remain_c && current_a < 2)
                    || current_b == 2
                    || current_c == 2)
            {
                result.push('a');
                remain_a -= 1;
                current_a += 1;
                current_b = 0;
                current_c = 0;
            } else if remain_b > 0
                && ((remain_b >= remain_a && remain_b >= remain_c && current_b < 2)
                    || current_a == 2
                    || current_c == 2)
            {
                result.push('b');
                remain_b -= 1;
                current_b += 1;
                current_a = 0;
                current_c = 0;
            } else if remain_c > 0
                && ((remain_c >= remain_a && remain_c >= remain_b && current_c < 2)
                    || current_a == 2
                    || current_b == 2)
            {
                result.push('c');
                remain_c -= 1;
                current_c += 1;
                current_a = 0;
                current_b = 0;
            } else {
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            2 => b = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Longest diverse string: {}",
                    Solution::longest_diverse_string(a, b, c)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
