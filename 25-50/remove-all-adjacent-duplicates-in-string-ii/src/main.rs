use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut result: String = s;
        loop {
            let mut has_change: bool = false;
            let mut prefix: (char, i32) = (' ', 0);
            let mut next: String = String::new();
            for c in result.chars() {
                if c == prefix.0 {
                    prefix.1 += 1;
                    if prefix.1 == k {
                        has_change = true;
                        prefix.0 = ' ';
                        prefix.1 = 0;
                    }
                } else {
                    if prefix.0 != ' ' {
                        for _i in 0..prefix.1 {
                            next.push(prefix.0);
                        }
                    }
                    prefix.0 = c;
                    prefix.1 = 1;
                }
            }
            if prefix.0 != ' ' {
                for _i in 0..prefix.1 {
                    next.push(prefix.0);
                }
            }
            result = next;
            if !has_change {
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Remove all duplicates string: {}",
                    Solution::remove_duplicates(s, k)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
