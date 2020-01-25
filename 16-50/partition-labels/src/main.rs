use std::env;

struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut positions: Vec<(usize, usize)> = vec![(len, len); 26];
        for (i, b) in bytes.iter().enumerate() {
            if positions[*b as usize - 97].0 == len {
                positions[*b as usize - 97].0 = i;
            }
            positions[*b as usize - 97].1 = i;
        }
        let mut parts: Vec<(usize, usize)> = Vec::new();
        for (i, b) in bytes.iter().enumerate() {
            match parts.last_mut() {
                Some(x) => {
                    if i <= (*x).1 {
                        if positions[*b as usize - 97].1 > (*x).1 {
                            (*x).1 = positions[*b as usize - 97].1;
                        }
                    } else {
                        parts.push(positions[*b as usize - 97]);
                    }
                }
                None => {
                    parts.push(positions[*b as usize - 97]);
                }
            }
        }
        let mut result: Vec<i32> = Vec::new();
        for part in parts {
            result.push((part.1 - part.0 + 1) as i32);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = 1;
            let s: String = arg;
            let result: Vec<i32> = Solution::partition_labels(s);
            for r in result {
                print!("{} ", r);
            }
            println!();
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
