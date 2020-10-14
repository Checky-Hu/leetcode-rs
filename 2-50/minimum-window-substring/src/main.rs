use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut target: HashMap<u8, i32> = HashMap::new();
        for u in t.bytes() {
            match target.get_mut(&u) {
                Some(x) => *x += 1,
                None => {
                    target.insert(u, 1);
                }
            }
        }
        let number: usize = target.len();
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut source: HashMap<u8, i32> = HashMap::new();
        let mut count: usize = 0;
        let mut left: usize = 0;
        let mut right: usize = 0;
        let (mut result_l, mut result_r) = (0, len);
        while right < len {
            if let Some(x) = target.get(&bytes[right]) {
                let t: i32 = match source.get_mut(&bytes[right]) {
                    Some(y) => {
                        *y += 1;
                        *y
                    }
                    None => {
                        source.insert(bytes[right], 1);
                        1
                    }
                };
                if t == *x {
                    count += 1;
                }
            }
            while left <= right && count == number {
                if right - left < result_r - result_l {
                    result_l = left;
                    result_r = right;
                }
                if let Some(x) = target.get(&bytes[left]) {
                    if let Some(y) = source.get_mut(&bytes[left]) {
                        *y -= 1;
                        if *y < *x {
                            count -= 1;
                        }
                    }
                }
                left += 1;
            }
            right += 1;
        }
        if result_r == len {
            String::new()
        } else {
            let mut result: String = String::new();
            for u in bytes.iter().take(result_r + 1).skip(result_l) {
                result.push(*u as char);
            }
            result
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let t: String = arg;
                println!("Min window substring: {}", Solution::min_window(s, t));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
