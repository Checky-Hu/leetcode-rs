use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut is_positive: bool = true;
        let mut is_inserted: bool = false;
        let x_char: char = (x as u8 + 48) as char;
        let mut result: String = String::new();
        for c in n.chars() {
            if c == '-' {
                is_positive = false;
            } else if !is_inserted && ((is_positive && x_char > c) || (!is_positive && x_char < c))
            {
                is_inserted = true;
                result.push(x_char);
            }
            result.push(c);
        }
        if !is_inserted {
            result.push(x_char);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = arg,
            _ => {
                ret += 1;
                let x: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Maximum value after insertion: {}",
                    Solution::max_value(n, x)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
