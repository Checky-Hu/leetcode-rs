use std::env;

struct Solution {}

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut is_block: bool = false;
        let mut out: String = String::new();
        for s in &source {
            let bytes: &[u8] = s.as_bytes();
            let len: usize = bytes.len();
            let mut i: usize = 0;
            while i < len {
                if is_block {
                    if i < len - 1 && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                        is_block = false;
                        i += 1;
                    }
                } else {
                    if i == len - 1 {
                        out.push(bytes[i] as char);
                    } else if bytes[i] == b'/' && bytes[i + 1] == b'*' {
                        is_block = true;
                        i += 1;
                    } else if bytes[i] == b'/' && bytes[i + 1] == b'/' {
                        break;
                    } else {
                        out.push(bytes[i] as char);
                    }
                }
                i += 1;
            }
            if !out.is_empty() && !is_block {
                result.push(out);
                out = String::new();
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut source: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                source.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<String> = Solution::remove_comments(source);
    for r in &result {
        println!("{}", *r);
    }
}
