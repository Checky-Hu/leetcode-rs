use std::env;

struct Solution {}

impl Solution {
    pub fn mask_pii(s: String) -> String {
        let bytes: &[u8] = s.as_bytes();
        let mut result: String = String::new();
        if bytes[0].is_ascii_alphabetic() {
            result.push(bytes[0].to_ascii_lowercase() as char);
            result.push_str("*****");
            let mut meet_at: bool = false;
            for i in 1..bytes.len() {
                if meet_at {
                    result.push(bytes[i].to_ascii_lowercase() as char);
                } else if bytes[i] == b'@' {
                    meet_at = true;
                    result.push(bytes[i - 1].to_ascii_lowercase() as char);
                    result.push('@');
                }
            }
        } else {
            let mut digits: Vec<u8> = Vec::new();
            for u in bytes {
                if u.is_ascii_digit() {
                    digits.push(*u);
                }
            }
            let len: usize = digits.len();
            if len > 10 {
                result.push('+');
                for _i in 0..(len - 10) {
                    result.push('*');
                }
                result.push('-');
            }
            for _i in 0..2 {
                result.push_str("***-");
            }
            for u in digits.iter().take(len).skip(len - 4) {
                result.push(*u as char);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Masking information: {}", Solution::mask_pii(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
