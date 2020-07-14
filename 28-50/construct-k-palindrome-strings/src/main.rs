use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let bytes: &[u8] = s.as_bytes();
        let len: i32 = bytes.len() as i32;
        let mut counts: Vec<i32> = vec![0; 26];
        for u in bytes {
            counts[*u as usize - 97] += 1;
        }
        let mut odd: i32 = 0;
        for count in counts.iter() {
            if count & 1 == 1 {
                odd += 1;
            }
        }
        odd <= k && k <= len
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
                println!(
                    "Can construct {} palindrome strings: {}",
                    k,
                    Solution::can_construct(s, k)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
