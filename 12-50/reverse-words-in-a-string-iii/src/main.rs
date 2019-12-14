use std::env;

struct Solution {
}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut result: String = String::with_capacity(len);
        let mut i: usize = 0;
        let mut reach_s: bool = true;
        let mut cur_s: usize = 0;
        while i < len {
            if bytes[i] == 32 {
                let range: usize = i - cur_s;
                for j in 1..=range {
                    result.push(bytes[i - j] as char);
                }
                result.push(' ');
                reach_s = true;
            } else {
                if reach_s {
                    cur_s = i;
                    reach_s = false;
                }
            }
            i += 1;
        }
        let range: usize = i - cur_s;
        for j in 1..=range {
            result.push(bytes[i - j] as char);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Reverse words: {}", Solution::reverse_words(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
