use std::env;

struct Solution {}

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut left: Vec<i32> = Vec::with_capacity(len + 1);
        let mut count: i32 = 0;
        for b in bytes.iter() {
            left.push(count);
            if *b == 98 {
                count += 1;
            }
        }
        left.push(count);
        count = 0;
        let mut right: Vec<i32> = vec![0; len + 1];
        let mut i: usize = len - 1;
        loop {
            right[i + 1] = count;
            if bytes[i] == 97 {
                count += 1;
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        right[0] = count;
        let mut result: i32 = -1;
        for j in 0..=len {
            let t: i32 = left[j] + right[j];
            if result < 0 || t < result {
                result = t;
            }
        }
        result
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
                println!("Minimum deletions: {}", Solution::minimum_deletions(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
