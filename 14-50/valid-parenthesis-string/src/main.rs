use std::env;

struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '*' => stack.push(c),
                ')' => {
                    let len: usize = stack.len();
                    if len == 0 {
                        return false;
                    }
                    let mut i: usize = len - 1;
                    loop {
                        if stack[i] == '(' {
                            break;
                        } else if i == 0 {
                            i = len;
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                    if i < len {
                        stack.remove(i);
                    } else {
                        stack.remove(len - 1);
                    }
                }
                _ => (),
            }
        }
        let mut lp_count: i32 = 0;
        for c in &stack {
            if *c == '(' {
                lp_count += 1;
            } else if lp_count > 0 {
                lp_count -= 1;
            }
        }
        lp_count == 0
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = 1;
            let s: String = arg;
            println!("Valid: {}", Solution::check_valid_string(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
