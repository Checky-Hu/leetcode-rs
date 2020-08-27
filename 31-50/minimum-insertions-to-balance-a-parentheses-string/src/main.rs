use std::env;

struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        let mut count: i32 = 0;
        let mut result: i32 = 0;
        for c in s.chars() {
            if c == '(' {
                if count > 0 {
                    while count >= 2 && !stack.is_empty() {
                        count -= 2;
                        stack.pop();
                    }
                    if stack.is_empty() {
                        result += count / 2 + (count & 1) * 2;
                    } else if count == 1 {
                        stack.pop();
                        result += 1;
                    }
                    count = 0;
                }
                stack.push('(');
            } else {
                count += 1;
            }
        }
        if count > 0 {
            while count >= 2 && !stack.is_empty() {
                count -= 2;
                stack.pop();
            }
            if stack.is_empty() {
                result += count / 2 + (count & 1) * 2;
            } else if count == 1 {
                stack.pop();
                result += 1;
            }
        }
        result += stack.len() as i32 * 2;
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Minimum insertions: {}", Solution::min_insertions(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
