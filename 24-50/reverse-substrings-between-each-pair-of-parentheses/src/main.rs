use std::env;

struct Solution {}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack: Vec<String> = Vec::new();
        let mut current: String = String::new();
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(current);
                    current = String::new();
                }
                ')' => {
                    let mut vec: Vec<u8> = current.into_bytes();
                    vec.reverse();
                    current = String::from_utf8(vec).unwrap_or_default();
                    if let Some(mut x) = stack.pop() {
                        x.push_str(&current);
                        current = x;
                    }
                }
                _ => current.push(c),
            }
        }
        current
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Reverse parentheses: {}", Solution::reverse_parentheses(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
