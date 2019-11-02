use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut result: i32 = 0;
        let mut max: i32 = 0;
        let mut start: usize = 0;
        let mut v: Vec<i32> = vec![0; 26];
        let bytes: &[u8] = s.as_bytes();
        for i in 0..bytes.len() {
            let tmp_u: usize = (bytes[i] - 65) as usize;
            v[tmp_u] += 1;
            if v[tmp_u] > max {
                max = v[tmp_u];
            }
            let mut tmp_i: i32 = (i - start) as i32 + 1;
            while tmp_i - max > k {
                v[(bytes[start] - 65) as usize] -= 1;
                start += 1;
                tmp_i -= 1;
            }
            if tmp_i > result {
                result = tmp_i;
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
        } else if 2 == index {
            ret += 1;
            let k: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Max length after replacement: {}", Solution::character_replacement(s, k));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
