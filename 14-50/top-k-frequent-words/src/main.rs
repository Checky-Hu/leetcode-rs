extern crate quicksort;

use quicksort::qstuple;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut s_c_map: HashMap<String, i32> = HashMap::new();
        for word in words {
            match s_c_map.get_mut(&word) {
                Some(x) => *x += 1,
                None => {
                    s_c_map.insert(word, 1);
                }
            }
        }
        let len: usize = s_c_map.len();
        let mut counts: Vec<(i32, String)> = Vec::with_capacity(len);
        for (k, v) in s_c_map.iter() {
            counts.push((*v, k.to_string()));
        }
        qstuple::quick_sort_by_func(&mut counts, 0, len - 1);
        let mut result: Vec<String> = Vec::new();
        for i in 0..k {
            result.push(counts[i as usize].1.to_string());
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let s: String = arg;
                words.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<String> = Solution::top_k_frequent(words, k);
    for r in &result {
        println!("{}", *r);
    }
}
