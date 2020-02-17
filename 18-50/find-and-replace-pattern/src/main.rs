use std::env;

struct Solution {}

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut word_pattern: Vec<usize> = vec![256; 256];
        let mut pattern_word: Vec<usize> = vec![256; 256];
        let mut match_pattern: bool = true;
        let mut result: Vec<String> = Vec::new();
        let p_bytes: &[u8] = pattern.as_bytes();
        for word in words {
            for (i, u) in word.as_bytes().iter().enumerate() {
                if word_pattern[*u as usize] == 256 {
                    if pattern_word[p_bytes[i] as usize] == 256 {
                        word_pattern[*u as usize] = p_bytes[i] as usize;
                        pattern_word[p_bytes[i] as usize] = *u as usize;
                    } else if pattern_word[p_bytes[i] as usize] != *u as usize {
                        match_pattern = false;
                        break;
                    }
                } else if word_pattern[*u as usize] != p_bytes[i] as usize {
                        match_pattern = false;
                        break;
                }
            }
            if match_pattern {
                result.push(word);
            } else {
                match_pattern = true;
            }
            for v in word_pattern.iter_mut() {
                *v = 256;
            }
            for v in pattern_word.iter_mut() {
                *v = 256;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut pattern: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => pattern = arg,
            _ => {
                ret += 1;
                let word: String = arg;
                words.push(word);
            }
        }
    }

    if pattern.is_empty() || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<String> = Solution::find_and_replace_pattern(words, pattern);
    for r in &result {
        println!("{}", *r);
    }
}
