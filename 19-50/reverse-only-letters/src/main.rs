use std::env;

struct Solution {
}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let s_bytes: &[u8] = s.as_bytes();
        let len: usize = s_bytes.len();
        if len == 0 {
            return String::new()
        }

        let mut result: Vec<u8> = vec![0; len];
        let mut start: usize = 0;
        let mut end: usize = len - 1;
        while start <= end {
            while start < end {
                if s_bytes[start] >= 65 && s_bytes[start] <= 90 {
                    break;
                } else if s_bytes[start] >= 97 && s_bytes[start] <= 122 {
                    break;
                } else {
                    result[start] = s_bytes[start];
                    start += 1;
                }
            }
            while start < end {
                if s_bytes[end] >= 65 && s_bytes[end] <= 90 {
                    break;
                } else if s_bytes[end] >= 97 && s_bytes[end] <= 122 {
                    break;
                } else {
                    result[end] = s_bytes[end];
                    end -= 1;
                }
            }
            if start < end {
                result[start] = s_bytes[end];
                result[end] = s_bytes[start];
                start += 1;
                end -= 1;
            } else if start == end {
                result[start] = s_bytes[start];
                break;
            }
        }
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Reverse: {}", Solution::reverse_only_letters(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

