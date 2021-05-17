use std::env;

struct Solution {}

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut vec: Vec<String> = vec![String::new(); 9];
        let mut result: String = String::new();
        let mut index: usize;
        for c in s.chars() {
            if c.is_ascii_digit() {
                index = c as usize - 49;
                vec[index] = result;
                result = String::new();
            } else if c.is_ascii_alphabetic() {
                result.push(c);
            }
        }
        for v in vec.iter() {
            if v.is_empty() {
                break;
            } else {
                result.push_str(v);
                result.push(' ');
            }
        }
        result.pop();
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!("Sorting sentence: {}", Solution::sort_sentence(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
