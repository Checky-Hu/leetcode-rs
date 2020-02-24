use std::env;

struct Solution {}

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut tmp: Vec<i32> = Vec::new();
        let mut pre: (u8, i32) = (0, 0);
        for u in s.as_bytes() {
            let t: u8 = u - 48;
            if pre.0 == t {
                pre.1 += 1;
            } else {
                tmp.push(pre.1);
                pre.0 = t;
                pre.1 = 1;
            }
        }
        tmp.push(pre.1);
        let mut right_0: i32 = 0;
        let len: usize = tmp.len();
        let mut i: usize = 2;
        while i < len {
            right_0 += tmp[i];
            i += 2;
        }
        let mut left_1: i32 = 0;
        let mut result: i32 = -1;
        i = 0;
        while i < len {
            let t: i32 = left_1 + right_0;
            if result == -1 || t < result {
                result = t;
            }
            if i + 1 < len {
                left_1 += tmp[i + 1];
            }
            if i + 2 < len {
                right_0 -= tmp[i + 2];
            }
            i += 2;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Min flips to monotone increasing: {}",
                Solution::min_flips_mono_incr(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
