use std::env;

struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let len: usize = s.len();
        let mut state: Vec<bool> = vec![false; len];
        let mut stack: Vec<(char, usize)> = Vec::with_capacity(len);
        for (i, c) in s.chars().enumerate() {
            if c == ')' {
                if let Some(x) = stack.last() {
                    if x.0 == '(' {
                        state[x.1] = true;
                        state[i] = true;
                    }
                    stack.pop();
                }
            } else {
                stack.push((c, i));
            }
        }
        let mut result: i32 = 0;
        let mut counts: i32 = 0;
        for v in state.iter() {
            if *v {
                counts += 1;
            } else {
                if counts > result {
                    result = counts;
                }
                counts = 0;
            }
        }
        if counts > result {
            counts
        } else {
            result
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret += 1;
            println!(
                "Longest valid parentheses: {}",
                Solution::longest_valid_parentheses(arg)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
