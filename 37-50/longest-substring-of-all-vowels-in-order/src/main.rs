use std::env;

struct Solution {}

impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut result: i32 = 0;
        let mut status: (char, usize, usize) = ('b', 0, 0);
        for (i, c) in word.chars().enumerate() {
            let is_valid: bool = match status.0 {
                'a' => c == 'a' || c == 'e',
                'e' => c == 'e' || c == 'i',
                'i' => c == 'i' || c == 'o',
                'o' => c == 'o' || c == 'u',
                'u' => c == 'u',
                _ => false,
            };
            if is_valid {
                status.0 = c;
                status.2 = i;
            } else {
                if status.0 == 'u' {
                    let t: i32 = (status.2 - status.1) as i32 + 1;
                    if t > result {
                        result = t;
                    }
                }
                if c == 'a' {
                    status.0 = 'a';
                } else {
                    status.0 = 'b';
                }
                status.1 = i;
                status.2 = i;
            }
            status.2 = i;
        }
        if status.0 == 'u' {
            let t: i32 = (status.2 - status.1) as i32 + 1;
            if t > result {
                result = t;
            }
        }
        result
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
                    "Longest beautiful substring: {}",
                    Solution::longest_beautiful_substring(word)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
