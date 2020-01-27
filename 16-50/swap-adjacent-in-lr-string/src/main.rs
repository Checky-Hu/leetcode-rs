use std::env;

struct Solution {}

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let s_bytes: &[u8] = start.as_bytes();
        let e_bytes: &[u8] = end.as_bytes();
        let len: usize = s_bytes.len();
        let mut count_l: i32 = 0;
        let mut count_r: i32 = 0;
        for i in 0..len {
            if s_bytes[i] == 82 {
                count_r += 1;
            }
            if e_bytes[i] == 76 {
                count_l += 1;
            }
            if s_bytes[i] == 76 {
                count_l -= 1;
            }
            if e_bytes[i] == 82 {
                count_r -= 1;
            }
            if count_l < 0 || count_r < 0 || count_l * count_r != 0 {
                return false;
            }
        }
        count_l == 0 && count_r == 0
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut start: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => start = arg,
            _ => {
                ret += 1;
                let end: String = arg;
                println!("Transform: {}", Solution::can_transform(start, end));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
