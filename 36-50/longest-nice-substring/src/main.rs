use std::env;

struct Solution {}

impl Solution {
    fn is_nice(counts: &[i32]) -> bool {
        for v in counts.iter() {
            if *v == 1 || *v == 2 {
                return false;
            }
        }
        true
    }

    pub fn longest_nice_substring(s: String) -> String {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut result: String = String::new();
        for i in 0..len {
            // 0 => None, 1 => low, 2 => high
            let mut counts: Vec<i32> = vec![0; 26];
            let mut string: String = String::new();
            for j in i..len {
                string.push(bytes[j] as char);
                if 65 <= bytes[j] && bytes[j] <= 90 {
                    counts[bytes[j] as usize - 65] |= 2;
                } else {
                    counts[bytes[j] as usize - 97] |= 1;
                }
                if Solution::is_nice(&counts) && string.len() > result.len() {
                    result = string.clone();
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!(
                    "Longest nice substring: {}",
                    Solution::longest_nice_substring(s)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
