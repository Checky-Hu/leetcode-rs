use std::env;

struct Solution {
}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut count: i32 = 0;
        let mut result: String = String::new();
        let mut tmp: String = String::new();
        for c in s.chars() {
            if c == '(' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                tmp.remove(0);
                result.push_str(&tmp);
                tmp = String::new();
            } else {
                tmp.push(c);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
	            ret = 1;
                let s: String = arg;
                println!("String: {}", Solution::remove_outer_parentheses(s));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

