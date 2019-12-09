use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut result: String = String::with_capacity(len);
        let mut i: usize = 0;
        while i < len {
            let range: usize = if i + k as usize - 1 < len {
                k as usize - 1
            } else {
                len - 1 - i
            };
            let mut j: usize = 0;
            while j <= range {
                result.push(bytes[i + range - j] as char);
                j += 1;
            }
            i += j;
            j = 1;
            while j <= k as usize && i < len {
                result.push(bytes[i] as char);
                i += 1;
                j += 1;
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
            ret += 1;
            s = arg;
        } else if 2 == index {
            ret += 1;
            let k: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("After {} reverse: {}", k, Solution::reverse_str(s, k));
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
