use std::env;

struct Solution {}

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut vec: Vec<bool> = vec![false; 26];
        for c in allowed.chars() {
            vec[c as usize - 97] = true;
        }
        let mut result: i32 = 0;
        for word in words.iter() {
            let mut is_allowed: bool = true;
            for c in word.chars() {
                if !vec[c as usize - 97] {
                    is_allowed = false;
                    break;
                }
            }
            if is_allowed {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut allowed: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => allowed = arg,
            _ => {
                ret += 1;
                words.push(arg);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Consistent strings: {}",
        Solution::count_consistent_strings(allowed, words)
    );
}
