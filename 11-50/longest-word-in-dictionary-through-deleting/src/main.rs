extern crate quicksort;

use std::env;
use quicksort::qsstr;

struct Solution {
}

impl Solution {
    fn is_subsequence(a: &String, b: &String) -> bool {
        let a_len: usize = a.len();
        let a_bytes: &[u8] = a.as_bytes();
        let b_len: usize = b.len();
        let b_bytes: &[u8] = b.as_bytes();
        if a_len > b_len {
            return false
        }
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < a_len {
            let mut contains: bool = false;
            while j < b_len {
                if a_bytes[i] == b_bytes[j] {
                    contains = true;
                    break;
                }
                j += 1;
            }
            if contains {
                i += 1;
                j += 1;
            } else {
                return false
            }
        }
        true
    }

    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let empty_result: String = String::new();
        let len: usize = d.len();
        if len == 0 {
            return empty_result
        }
        let mut tmp: Vec<String> = d.clone();
        qsstr::quick_sort_by_func(&mut tmp, 0, len - 1);
        for r in tmp {
            if Solution::is_subsequence(&r, &s) {
                return r
            }
        }
        empty_result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    let mut d: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let tmp: String = arg;
                d.push(tmp);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return
    }

    println!("Longest word: {}", Solution::find_longest_word(s, d));
}
