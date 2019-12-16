use std::env;

struct Solution {
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let bytes1: &[u8] = s1.as_bytes();
        let len1: usize = bytes1.len();
        let bytes2: &[u8] = s2.as_bytes();
        let len2: usize = bytes2.len();
        if len1 > len2 {
            return false
        }

        let mut flags: Vec<i32> = vec![0; 26];
        let mut stats: Vec<i32> = vec![0; 26];
        let mut i: usize = 0;
        while i < len1 {
            flags[(bytes1[i] - 97) as usize] += 1;
            stats[(bytes2[i] - 97) as usize] += 1;
            i += 1;
        }
        let mut first_compare: bool = true;
        loop {
            let mut is_same: bool = true;
            for j in 0..26 {
                if flags[j] != stats[j] {
                    is_same = false;
                    break;
                }
            }
            if is_same {
                return true
            } else {
                if first_compare {
                    first_compare = false;
                } else {
                    i += 1;
                }
                if i == len2 {
                    break;
                } else {
                    stats[(bytes2[i - len1] - 97) as usize] -= 1;
                    stats[(bytes2[i] - 97) as usize] += 1;
                }
            }
        }
        false
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            s1 = arg;
        } else if 2 == index {
            ret += 1;
            let s2: String = arg;
            println!("Include: {}", Solution::check_inclusion(s1, s2));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
