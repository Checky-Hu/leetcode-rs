use std::env;

struct Solution {}

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut result: String = String::new();
        let len: usize = palindrome.len();
        if len == 1 {
            return result;
        }
        let is_odd: bool = (len & 1) != 0;
        let mid: usize = len / 2;
        let mut is_changed: bool = false;
        for (i, c) in palindrome.chars().enumerate() {
            if is_changed || (is_odd && i == mid) {
                result.push(c);
            } else if i == len - 1 {
                if c == 'a' {
                    result.push('b');
                } else {
                    result.push('a');
                }
            } else {
                if c != 'a' {
                    is_changed = true;
                }
                result.push('a');
            }
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
                let palindrome: String = arg;
                println!(
                    "Break palindrome: {}",
                    Solution::break_palindrome(palindrome)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
