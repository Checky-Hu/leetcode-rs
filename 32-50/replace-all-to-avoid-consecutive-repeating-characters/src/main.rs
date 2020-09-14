use std::env;

struct Solution {}

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut bytes: Vec<u8> = s.into_bytes();
        let len: usize = bytes.len();
        for i in 0..len {
            if bytes[i] == b'?' {
                let prefix: u8 = if i == 0 { 0 } else { bytes[i - 1] };
                let suffix: u8 = if i == len - 1 { 0 } else { bytes[i + 1] };
                for c in b'a'..=b'z' {
                    if c != prefix && c != suffix {
                        bytes[i] = c;
                        break;
                    }
                }
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Modified string: {}", Solution::modify_string(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
