use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut target: usize = k as usize;
        let bytes: &[u8] = s.as_bytes();
        let mut len: usize = 0;
        let mut index: usize = 0;
        for (i, u) in bytes.iter().enumerate() {
            if u.is_ascii_alphabetic() {
                len += 1;
            } else {
                len *= (*u - 48) as usize;
            }
            if len >= target {
                index = i;
                break;
            }
        }
        loop {
            if bytes[index].is_ascii_alphabetic() {
                if target % len == 0 {
                    break;
                } else {
                    len -= 1;
                }
            } else {
                len /= (bytes[index] - 48) as usize;
                target %= len;
            }
            if index == 0 {
                break;
            } else {
                index -= 1;
            }
        }
        (bytes[index] as char).to_string()
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Char at index: {}", Solution::decode_at_index(s, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
