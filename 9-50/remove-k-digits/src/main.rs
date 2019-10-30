use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut count: i32 = k;
        let mut result: String = String::new();
        for c in num.chars() {
            while count > 0 {
                match result.pop() {
                    Some(x) => {
                        if x > c {
                            count -= 1;
                        } else {
                            result.push(x);
                            break;
                        }
                    },
                    None => break,
                }
            }
            if !result.is_empty() || c != '0' {
                result.push(c);
            }
        }
        while !result.is_empty() && count > 0 {
            result.pop();
            count -= 1;
        }
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            k = i32::from_str(&arg).expect("Error parse.");
        } else if index == 2 {
            ret += 1;
            let num: String = arg;
            println!("Smallest num after removed {} digits: {}", k, Solution::remove_kdigits(num, k));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
