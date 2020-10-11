use std::env;

struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut bytes: Vec<u8> = s.into_bytes();
        while !bytes.is_empty() {
            if bytes[0] == b' ' {
                bytes.remove(0);
            } else {
                break;
            }
        }
        while let Some(x) = bytes.last() {
            if *x == b' ' {
                bytes.pop();
            } else {
                break;
            }
        }
        let mut has_exponent: bool = false;
        let mut has_number: bool = false;
        let mut has_point: bool = false;
        let mut has_sign: bool = false;
        for u in bytes.iter() {
            match *u {
                b'0'..=b'9' => has_number = true,
                b'e' => {
                    if !has_number || has_exponent {
                        return false;
                    } else {
                        has_exponent = true;
                        has_number = false;
                        has_point = false;
                        has_sign = false;
                    }
                }
                b'.' => {
                    if has_exponent || has_point {
                        return false;
                    } else {
                        has_point = true;
                    }
                }
                b'+' | b'-' => {
                    if has_number || has_point || has_sign {
                        return false;
                    } else {
                        has_sign = true;
                    }
                }
                _ => return false,
            }
        }
        has_number
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let s: String = arg;
            println!("Valid number: {}", Solution::is_number(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
