use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        let mut status: Vec<i32> = vec![0; 26];
        let s_bytes: &[u8] = s.as_bytes();
        let s_len: usize = s_bytes.len();
        let t_bytes: &[u8] = t.as_bytes();
        let t_len: usize = t_bytes.len();
        if s_len != t_len {
            return false;
        }
        for i in 0..s_len {
            let diff: usize =
                (if t_bytes[i] < s_bytes[i] { 26 } else { 0 } + t_bytes[i] - s_bytes[i]) as usize;
            status[diff] += 1;
        }
        for (i, v) in status.iter().enumerate().take(26).skip(1) {
            if *v > 0 {
                let t: i32 = (*v - 1) * 26 + i as i32;
                if t > k {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut t: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            2 => t = arg,
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Can convert string in k moves: {}",
                    Solution::can_convert_string(s, t, k)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
