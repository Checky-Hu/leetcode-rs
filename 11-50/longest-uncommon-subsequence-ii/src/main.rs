extern crate quicksort;

use std::collections::HashSet;
use std::env;
use quicksort::qsstr;

struct Solution {
}

impl Solution {
    fn is_subsequence(a: &String, b: &String) -> bool {
        println!("{} {}", a, b);
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
        println!("{} {}", i, j);
        true
    }

    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut once: HashSet<String> = HashSet::new();
        let mut more: HashSet<String> = HashSet::new();
        for s in &strs {
            if once.contains(s) {
                once.remove(s);
                more.insert(s.to_string());
            } else if !more.contains(s) {
                once.insert(s.to_string());
            }
        }
        let mut len: usize = 0;
        let mut tmp: Vec<String> = Vec::new();
        for s in once.drain() {
            println!("{}", s);
            tmp.push(s);
            len += 1;
        }
        if len > 1 {
            qsstr::quick_sort_by_descend_len(&mut tmp, 0, len - 1);
        }
        for a in &tmp {
            let mut is_ans: bool = true;
            for b in more.iter() {
                if Solution::is_subsequence(a, b) {
                    is_ans = false;
                    break;
                }
            }
            if is_ans {
                return a.len() as i32
            }
        }
        -1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut strs: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                strs.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Longest uncommon subsequence length: {}", Solution::find_lu_slength(strs));
}
