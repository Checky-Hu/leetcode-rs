use std::env;

struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == '(' {
                stack.push(c);
            } else if None == stack.pop() {
                result += 1;
            }
        }
        result + stack.len() as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Minimum add: {}", Solution::min_add_to_make_valid(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
