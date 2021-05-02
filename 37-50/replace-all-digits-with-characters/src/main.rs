use std::env;

struct Solution {}

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut bytes: Vec<u8> = s.into_bytes();
        let len: usize = bytes.len();
        let mut i: usize = 1;
        while i < len {
            bytes[i] = bytes[i - 1] + bytes[i] - 48;
            i += 2;
        }
        String::from_utf8(bytes).unwrap_or_default()
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!("Replace digits: {}", Solution::replace_digits(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
