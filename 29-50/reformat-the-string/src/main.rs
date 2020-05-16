use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn reformat(s: String) -> String {
        let len: usize = s.len();
        let mut c_index: usize = 0;
        let mut n_index: usize = 1;
        let mut result: Vec<u8> = vec![0; len];
        for c in s.chars() {
            if c.is_ascii_alphabetic() {
                if c_index >= len {
                    return String::new();
                } else {
                    result[c_index] = c as u8;
                    c_index += 2;
                }
            } else {
                match n_index.cmp(&len) {
                    Ordering::Greater => return String::new(),
                    Ordering::Equal => {
                        result.insert(0, c as u8);
                        result.pop();
                    }
                    Ordering::Less => {
                        result[n_index] = c as u8;
                        n_index += 2;
                    }
                }
            }
        }
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Reformat string: {}", Solution::reformat(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
