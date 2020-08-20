use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let bytes: &[u8] = s.as_bytes();
        let mut result: Vec<u8> = vec![0; bytes.len()];
        for (i, index) in indices.iter().enumerate() {
            result[*index as usize] = bytes[i];
        }
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut indices: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                indices.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Shuffle string: {}", Solution::restore_string(s, indices));
}
