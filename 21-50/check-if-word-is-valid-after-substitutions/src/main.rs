use std::env;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                'a' => stack.push(c),
                'b' => match stack.last() {
                    Some(x) => {
                        if *x == 'a' {
                            stack.push(c);
                        } else {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => match stack.pop() {
                    Some(x) => {
                        if x != 'b' {
                            return false;
                        } else {
                            match stack.pop() {
                                Some(y) => {
                                    if y != 'a' {
                                        return false;
                                    }
                                }
                                None => return false,
                            }
                        }
                    }
                    None => return false,
                },
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let s: String = arg;
            println!("Valid string: {}", Solution::is_valid(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
