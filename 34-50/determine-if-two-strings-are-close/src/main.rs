use std::env;

struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let mut vec1: Vec<i32> = vec![0; 26];
        for c in word1.chars() {
            vec1[c as usize - 97] += 1;
        }
        let mut vec2: Vec<i32> = vec![0; 26];
        for c in word2.chars() {
            vec2[c as usize - 97] += 1;
        }
        for i in 0..26 {
            if vec1[i] == 0 {
                continue;
            }
            let mut found: bool = false;
            for (j, v_mut) in vec2.iter_mut().enumerate() {
                if *v_mut == vec1[i] && vec1[j] != 0 {
                    found = true;
                    *v_mut = 0;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true
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
                println!("Close strings: {}", Solution::close_strings(word1, word2));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
