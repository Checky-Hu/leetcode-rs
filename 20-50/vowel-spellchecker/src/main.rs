use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let len: usize = wordlist.len();
        let mut result: Vec<String> = Vec::new();
        for query in queries.iter() {
            let q_len: usize = query.len();
            let q_bytes: &[u8] = query.as_bytes();
            // first value: index of match pattern.
            // second value: 0 => not found, 1 => vowel, 2 => case.
            let mut pre: (usize, u8) = (len, 0);
            for (index, word) in wordlist.iter().enumerate() {
                let w_len: usize = word.len();
                if q_len != w_len {
                    continue;
                }
                let mut is_same: bool = true;
                let mut is_case: bool = true;
                let mut is_vowel: bool = true;
                let w_bytes: &[u8] = word.as_bytes();
                for i in 0..w_len {
                    if w_bytes[i] == q_bytes[i] {
                        continue;
                    } else if w_bytes[i] + 32 == q_bytes[i] || w_bytes[i] - 32 == q_bytes[i] {
                        is_same = false;
                    } else {
                        is_same = false;
                        is_case = false;
                        let f1: bool = match w_bytes[i] {
                            b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => {
                                true
                            }
                            _ => false,
                        };
                        let f2: bool = match q_bytes[i] {
                            b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => {
                                true
                            }
                            _ => false,
                        };
                        if !f1 || !f2 {
                            is_vowel = false;
                            break;
                        }
                    }
                }
                if is_same {
                    pre.0 = index;
                    break;
                } else if is_case && pre.1 < 2 {
                    pre.0 = index;
                    pre.1 = 2;
                } else if is_vowel && pre.1 < 1 {
                    pre.0 = index;
                    pre.1 = 1;
                }
            }
            if pre.0 == len {
                result.push(String::new());
            } else {
                result.push(wordlist[pre.0].clone());
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut wordlist: Vec<String> = Vec::new();
    let mut queries: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let s: String = arg;
                if wordlist.len() == n as usize {
                    queries.push(s);
                } else {
                    wordlist.push(s);
                }
            }
        }
    }

    if 0 == n || n >= ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result: Vec<String> = Solution::spellchecker(wordlist, queries);
    for s in result.iter() {
        println!("{}", *s);
    }
}
