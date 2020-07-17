use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let bytes: &[u8] = s.as_bytes();
        let mask: i32 = (1 << k) - 1;
        let len: usize = mask as usize + 1;
        let mut visit: Vec<bool> = vec![false; len];
        // (bits, value)
        let mut state: (i32, i32) = (0, 0);
        for i in 0..bytes.len() {
            state.1 = bytes[i] as i32 - 48
                + if state.0 == k {
                    if bytes[i - k as usize] == b'1' {
                        state.1 << 1 & mask
                    } else {
                        state.1 << 1
                    }
                } else {
                    state.0 += 1;
                    state.1 << 1
                };
            if state.0 == k {
                visit[state.1 as usize] = true;
            }
        }
        for v in visit.iter() {
            if !*v {
                return false;
            }
        }
        true
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
                println!("Has all codes: {}", Solution::has_all_codes(s, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
