extern crate mystring;

use mystring::stringop;
use std::env;

struct Solution {
}

impl Solution {
    pub fn check(s_bytes: &[u8], index: usize, v: &mut Vec<String>, result: &mut bool) {
        if *result {
            return
        }
        if index >= s_bytes.len() {
            if v.len() >= 3 {
                *result = true;
            }
            return
        }
        for i in index..s_bytes.len() {
            if i - index > 0 && s_bytes[index] == 48 {
                break;
            }
            let tmp: String = String::from_utf8(s_bytes[index..=i].to_vec()).unwrap();
            let len: usize = v.len();
            if len >= 2 && stringop::add_string(&v[len - 2], &v[len - 1]) != tmp {
                continue;
            }
            v.push(tmp);
            Solution::check(s_bytes, i + 1, v, result);
            v.pop();
        }
    }

    pub fn is_additive_number(num: String) -> bool {
        let mut result: bool = false;
        let mut v: Vec<String> = Vec::new();
        Solution::check(num.as_bytes(), 0, &mut v, &mut result);
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
                ret += 1;
                let num: String = arg;
                println!("Additive number: {}", Solution::is_additive_number(num));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
