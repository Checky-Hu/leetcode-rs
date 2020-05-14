use std::env;

struct Solution {}

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut strs: Vec<String> = words;
        strs.sort_by(|a, b| b.len().cmp(&a.len()));
        let mut result: Vec<String> = Vec::new();
        for c in strs.iter() {
            for s in strs.iter() {
                if s.len() <= c.len() {
                    break;
                } else if s.contains(c) {
                    result.push(c.clone());
                    break;
                }
            }
        }
        result
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

    let result: Vec<String> = Solution::string_matching(words);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
