use std::env;

struct Solution {
}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut flags: Vec<i32> = vec![0; 26];
        for c in chars.chars() {
            flags[(c as u8 - 97) as usize] += 1;
        }
        let mut result: i32 = 0;
        for s in words {
            let mut tmp: Vec<i32> = vec![0; 26];
            for c in s.chars() {
                tmp[(c as u8 - 97) as usize] += 1;
            }
            let mut is_valid: bool = true;
            for i in 0..26 {
                if tmp[i] > flags[i] {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                result += s.len() as i32;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut chars: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => chars = arg,
            _ => {
                ret += 1;
                let word: String = arg;
                words.push(word);
            },
        }
    }

    if chars.is_empty() || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Characters: {}", Solution::count_characters(words, chars));
}
