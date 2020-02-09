use std::env;

struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut c_stack: Vec<usize> = Vec::new();
        let mut n_stack: Vec<(i32, usize)> = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                c_stack.push(i);
            } else {
                let pre_i: usize = c_stack.pop().unwrap();
                if pre_i + 1 == i {
                    n_stack.push((1, pre_i));
                } else {
                    let mut sum: i32 = 0;
                    while let Some(x) = n_stack.pop() {
                        if x.1 > pre_i {
                            sum += x.0;
                        } else {
                            n_stack.push((x.0, x.1));
                            break;
                        }
                    }
                    n_stack.push((sum * 2, pre_i));
                }
            }
        }
        let mut result: i32 = 0;
        while let Some(x) = n_stack.pop() {
            result += x.0;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Score of parentheses: {}",
                Solution::score_of_parentheses(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
