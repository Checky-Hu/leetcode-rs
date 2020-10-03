use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let v_len: usize = words.len();
        let w_len: usize = words[0].len();
        let total: usize = v_len * w_len;
        let bytes: &[u8] = s.as_bytes();
        let s_len: usize = bytes.len();
        if s_len < total {
            return result;
        }
        let mut target: HashMap<String, i32> = HashMap::new();
        for word in words {
            match target.get_mut(&word) {
                Some(x) => *x += 1,
                None => {
                    target.insert(word, 1);
                }
            }
        }
        for i in 0..=(s_len - total) {
            let mut is_match: bool = true;
            let mut current: HashMap<String, i32> = target.clone();
            let mut j: usize = i;
            while j < i + total {
                let mut tmp: String = String::new();
                for k in 0..w_len {
                    tmp.push(bytes[j + k] as char);
                }
                match current.get_mut(&tmp) {
                    Some(x) => {
                        if *x > 0 {
                            *x -= 1;
                        } else {
                            is_match = false;
                            break;
                        }
                    }
                    None => {
                        is_match = false;
                        break;
                    }
                }
                j += w_len;
            }
            if is_match {
                result.push(i as i32);
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                words.push(arg);
            }
        }
    }

    if ret == 0 {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<i32> = Solution::find_substring(s, words);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
