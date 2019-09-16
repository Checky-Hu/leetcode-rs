use std::collections::HashSet;
use std::env;

struct Solution {
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let len: usize = s.len();
        let s_bytes: &[u8] = s.as_bytes();
        let mut flags: Vec<bool> = vec![false; len + 1];
        flags[0] = true;
        for i in 1..=len {
            let mut j: usize = i - 1;
            let mut tmp: String = String::new();
            tmp.push(s_bytes[j] as char);
            loop {
                if flags[j] && word_set.contains(&tmp) {
                    flags[i] = true;
                    break;
                } else {
                    if j == 0 {
                        break;
                    } else {
                        j -= 1;
                        tmp.insert(0, s_bytes[j] as char);
                    }
                }
            }
        }
        flags[len]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut word_dict: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let tmp: String = arg;
                word_dict.push(tmp);
            },
        }
    }

    if ret == 0 {
        println!("Require at least two parameters.");
        return;
    }

    println!("Word break: {}", Solution::word_break(s, word_dict));
}
