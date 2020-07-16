use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        // (count, number)
        let mut status: (i32, i32) = (0, 0);
        let mut result: i32 = 0;
        for i in 0..len {
            match bytes[i] {
                b'a' | b'e' | b'i' | b'o' | b'u' => status.1 += 1,
                _ => (),
            }
            if status.0 == k {
                match bytes[i - k as usize] {
                    b'a' | b'e' | b'i' | b'o' | b'u' => status.1 -= 1,
                    _ => (),
                }
            } else {
                status.0 += 1;
            }
            if status.1 > result {
                result = status.1;
            }
        }
        result
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
                println!("Max vowels: {}", Solution::max_vowels(s, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
