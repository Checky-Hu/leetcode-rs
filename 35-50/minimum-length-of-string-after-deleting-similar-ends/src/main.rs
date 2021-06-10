use std::env;

struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let bytes: Vec<u8> = s.into_bytes();
        let len: usize = bytes.len();
        let mut left: usize = 0;
        let mut right: usize = len - 1;
        loop {
            if bytes[left] == bytes[right] {
                if left == right {
                    return 1;
                } else {
                    while left < right && bytes[left] == bytes[right] {
                        left += 1;
                    }
                    if left == right {
                        return 0;
                    } else {
                        while left <= right && bytes[left - 1] == bytes[right] {
                            right -= 1;
                        }
                    }
                }
            } else {
                break;
            }
        }
        (right + 1 - left) as i32
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
                    "Minimum length of string after deleting similar ends: {}",
                    Solution::minimum_length(s)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
