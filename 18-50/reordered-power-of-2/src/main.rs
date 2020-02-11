use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(
        bytes: &[u8],
        index: usize,
        len: usize,
        flags: &mut Vec<bool>,
        result: &mut String,
    ) -> bool {
        if index == len {
            let mut t: i32 = i32::from_str(result).unwrap_or(0);
            while t != 1 {
                if t & 1 == 0 {
                    t >>= 1;
                } else {
                    return false;
                }
            }
            true
        } else {
            for i in 0..len {
                if flags[i] || (index == 0 && bytes[i] == b'0') {
                    continue;
                }
                flags[i] = true;
                result.push(bytes[i] as char);
                if Solution::helper(bytes, index + 1, len, flags, result) {
                    return true;
                }
                result.pop();
                flags[i] = false;
            }
            false
        }
    }

    pub fn reordered_power_of2(n: i32) -> bool {
        let s: String = n.to_string();
        let len: usize = s.len();
        let mut flags: Vec<bool> = vec![false; len];
        let mut result: String = String::new();
        Solution::helper(s.as_bytes(), 0, len, &mut flags, &mut result)
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Power of 2 after reorder: {}",
                Solution::reordered_power_of2(n)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
