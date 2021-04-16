use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut set: HashSet<String> = HashSet::new();
        let mut tmp: String = String::new();
        for c in word.chars() {
            if c.is_numeric() {
                if let Some(x) = tmp.chars().next() {
                    if x == '0' {
                        tmp.pop();
                    }
                }
                tmp.push(c);
            } else if !tmp.is_empty() {
                set.insert(tmp);
                tmp = String::new();
            }
        }
        if !tmp.is_empty() {
            set.insert(tmp);
        }
        set.len() as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let word: String = arg;
                println!(
                    "Number of different integers: {}",
                    Solution::num_different_integers(word)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
