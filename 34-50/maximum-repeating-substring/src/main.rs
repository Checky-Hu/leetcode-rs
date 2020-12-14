use std::env;

struct Solution {}

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let s_bytes: &[u8] = sequence.as_bytes();
        let s_len: usize = s_bytes.len();
        let w_bytes: &[u8] = word.as_bytes();
        let w_len: usize = w_bytes.len();
        let mut result: usize = 0;
        let mut current: usize = 0;
        let mut s_i: usize = 0;
        let mut w_i: usize = 0;
        while s_i < s_len {
            if s_bytes[s_i] == w_bytes[w_i] {
                s_i += 1;
                w_i += 1;
                if w_i == w_len {
                    w_i = 0;
                    current += 1;
                }
            } else {
                s_i = s_i + 1 - current * w_len - w_i;
                w_i = 0;
                if current > result {
                    result = current;
                }
                current = 0;
            }
        }
        if current > result {
            result = current;
        }
        result as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut sequence: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => sequence = arg,
            _ => {
                ret += 1;
                let word: String = arg;
                println!(
                    "Maximum repeating: {}",
                    Solution::max_repeating(sequence, word)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
