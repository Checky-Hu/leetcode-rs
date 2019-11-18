use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut result: String = String::new();
        let len: usize = s.len();
        if len == 0 {
            return result
        }
        let bytes: &[u8] = s.as_bytes();
        let mut count: i32 = 0;
        let mut i: usize = len - 1;
        loop {
            if bytes[i] != 45 {
                if count == k {
                    result.insert(0, '-');
                    count = 0;
                }
                if 97 <= bytes[i] && bytes[i] <= 122 {
                    result.insert(0, (bytes[i] - 32) as char);
                } else {
                    result.insert(0, bytes[i] as char);
                }
                count += 1;
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            s = arg;
        } else if 1 < index {
            ret += 1;
            let k: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("License after formatting: {}", Solution::license_key_formatting(s, k));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }
}
