use std::env;

struct Solution {}

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        // (x, y)
        let mut diff: (i32, i32) = (0, 0);
        let bytes1: &[u8] = s1.as_bytes();
        let bytes2: &[u8] = s2.as_bytes();
        let len: usize = bytes1.len();
        for i in 0..len {
            if bytes1[i] != bytes2[i] {
                if bytes1[i] == b'x' {
                    diff.0 += 1;
                } else {
                    diff.1 += 1;
                }
            }
        }
        if (diff.0 + diff.1) & 1 == 1 {
            -1
        } else {
            diff.0 / 2 + diff.1 / 2 + (diff.0 & 1) * 2
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s1 = arg,
            _ => {
                ret += 1;
                let s2: String = arg;
                println!("Minimum swaps: {}", Solution::minimum_swap(s1, s2));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
