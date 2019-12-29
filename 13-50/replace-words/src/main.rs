use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn replace_words(dict: Vec<String>, sentence: String) -> String {
        let mut result: String = String::new();
        let mut set: HashSet<String> = HashSet::new();
        for s in dict {
            set.insert(s);
        }
        let mut cur: String = String::new();
        let mut skip: bool = false;
        for c in sentence.chars() {
            if c >= 'a' && c <= 'z' {
                if !skip {
                    cur.push(c);
                    if set.contains(&cur) {
                        skip = true;
                    }
                }
            } else {
                if !cur.is_empty() {
                    result.push_str(&cur);
                    cur = String::new();
                }
                result.push(c);
                skip = false;
            }
        }
        if !cur.is_empty() {
            result.push_str(&cur);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut sentence: String = String::new();
    let mut dict: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => sentence = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                dict.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("After replace: {}", Solution::replace_words(dict, sentence));
}
