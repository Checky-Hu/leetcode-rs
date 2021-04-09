use std::env;

struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let bytes1: &[u8] = word1.as_bytes();
        let len1: usize = bytes1.len();
        let bytes2: &[u8] = word2.as_bytes();
        let len2: usize = bytes2.len();
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        let mut use_1: bool = true;
        let mut result: String = String::new();
        while i1 < len1 && i2 < len2 {
            result.push(if use_1 {
                i1 += 1;
                bytes1[i1 - 1] as char
            } else {
                i2 += 1;
                bytes2[i2 - 1] as char
            });
            use_1 = !use_1;
        }
        while i1 < len1 {
            result.push(bytes1[i1] as char);
            i1 += 1;
        }
        while i2 < len2 {
            result.push(bytes2[i2] as char);
            i2 += 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut word1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => word1 = arg,
            _ => {
                ret += 1;
                let word2: String = arg;
                println!(
                    "Merge alternately: {}",
                    Solution::merge_alternately(word1, word2)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
