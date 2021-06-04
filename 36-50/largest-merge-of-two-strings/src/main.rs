use std::env;

struct Solution {}

impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut result: String = String::new();
        let mut word1_mut: String = word1;
        let mut word2_mut: String = word2;
        while !word1_mut.is_empty() {
            let c: char = if word2_mut.is_empty() || word1_mut > word2_mut {
                word1_mut.remove(0)
            } else {
                word2_mut.remove(0)
            };
            result.push(c);
        }
        while !word2_mut.is_empty() {
            let c: char = word2_mut.remove(0);
            result.push(c);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut word1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => word1 = arg,
            _ => {
                ret += 1;
                let word2: String = arg;
                println!(
                    "Largest merge of two strings: {}",
                    Solution::largest_merge(word1, word2)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
