use std::env;

struct Solution {}

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut words: Vec<Vec<char>> = Vec::new();
        let mut max_len: usize = 0;
        let mut cur_len: usize = 0;
        let mut cur_vec: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == ' ' {
                if cur_len > max_len {
                    max_len = cur_len;
                }
                words.push(cur_vec);
                cur_len = 0;
                cur_vec = Vec::new();
            } else {
                cur_len += 1;
                cur_vec.push(c);
            }
        }
        if cur_len > max_len {
            max_len = cur_len;
        }
        words.push(cur_vec);
        let mut result: Vec<String> = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let mut tmp: String = String::new();
            let mut cur: String = String::new();
            for word in words.iter() {
                if word.len() <= i {
                    cur.push(' ');
                } else {
                    if !cur.is_empty() {
                        tmp.push_str(&cur);
                        cur = String::new();
                    }
                    tmp.push(word[i]);
                }
            }
            result.push(tmp);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                let result: Vec<String> = Solution::print_vertically(s);
                for r in result.iter() {
                    println!("{}", *r);
                }
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
