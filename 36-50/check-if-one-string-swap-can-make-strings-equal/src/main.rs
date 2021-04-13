use std::env;

struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut prefix: (u8, u8) = (0, 0);
        let bytes1: &[u8] = s1.as_bytes();
        let len: usize = bytes1.len();
        let bytes2: &[u8] = s2.as_bytes();
        for i in 0..len {
            if bytes1[i] != bytes2[i] {
                match prefix.0 {
                    0 => {
                        prefix.0 = bytes1[i];
                        prefix.1 = bytes2[i];
                    }
                    1 => return false,
                    _ => {
                        if prefix.0 == bytes2[i] && prefix.1 == bytes1[i] {
                            prefix.0 = 1;
                            prefix.1 = 1;
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        prefix.0 == prefix.1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s1 = arg,
            _ => {
                ret += 1;
                let s2: String = arg;
                println!("Almost equal: {}", Solution::are_almost_equal(s1, s2));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
