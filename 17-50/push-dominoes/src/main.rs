use std::env;

struct Solution {}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let bytes: &[u8] = dominoes.as_bytes();
        let len: usize = bytes.len();
        let mut result: Vec<u8> = vec![b'.'; len];
        let mut pre_d: (usize, u8) = (len, b'.');
        for (i, c) in bytes.iter().enumerate() {
            match *c {
                b'L' => {
                    match pre_d.1 {
                        b'.' => {
                            for v in result.iter_mut().take(i + 1) {
                                *v = b'L';
                            }
                        }
                        b'L' => {
                            for v in result.iter_mut().take(i + 1).skip(pre_d.0 + 1) {
                                *v = b'L';
                            }
                        }
                        b'R' => {
                            let mut l: usize = pre_d.0 + 1;
                            let mut r: usize = i - 1;
                            while l < r {
                                result[l] = b'R';
                                result[r] = b'L';
                                l += 1;
                                r -= 1;
                            }
                            result[i] = b'L';
                        }
                        _ => (),
                    }
                    pre_d.0 = i;
                    pre_d.1 = b'L';
                }
                b'R' => {
                    match pre_d.1 {
                        b'.' | b'L' => {
                            result[i] = b'R';
                        }
                        b'R' => {
                            for v in result.iter_mut().take(i + 1).skip(pre_d.0 + 1) {
                                *v = b'R';
                            }
                        }
                        _ => (),
                    }
                    pre_d.0 = i;
                    pre_d.1 = b'R';
                }
                _ => (),
            }
        }
        if pre_d.1 == b'R' {
            for v in result.iter_mut().take(len).skip(pre_d.0 + 1) {
                *v = b'R';
            }
        }
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let dominoes: String = arg;
            println!("Dominoes after push: {}", Solution::push_dominoes(dominoes));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
