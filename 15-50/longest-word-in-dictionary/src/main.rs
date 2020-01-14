extern crate quicksort;

use quicksort::qsstr;
use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut set: HashSet<String> = HashSet::new();
        for word in &words {
            set.insert(word.to_string());
        }
        let len: usize = words.len();
        let mut tmp: Vec<String> = words;
        qsstr::quick_sort_by_func(&mut tmp, 0, len - 1);
        for t in &tmp {
            let mut found: bool = true;
            let l: usize = t.len();
            for i in 1..l {
                if !set.contains(&t[0..i]) {
                    found = false;
                    break;
                }
            }
            if found {
                return t.to_string();
            }
        }
        "".to_string()
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                words.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Longest word: {}", Solution::longest_word(words));
}
