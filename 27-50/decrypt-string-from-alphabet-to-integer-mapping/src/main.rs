use std::env;

struct Solution {}

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut result: String = String::new();
        let mut i: usize = 0;
        while i < len {
            if (bytes[i] == b'1' || bytes[i] == b'2') && i + 2 < len && bytes[i + 2] == b'#' {
                let b: u8 = (bytes[i] - b'0') * 10 + bytes[i + 1] + 48;
                result.push(b as char);
                i += 3;
            } else {
                result.push((bytes[i] + 48) as char);
                i += 1;
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
            println!("Integer mapping: {}", Solution::freq_alphabets(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
